pub mod entities;
pub mod error;

use anyhow::Context;
use serde::Deserialize;
use tokio_postgres::{Config, NoTls, Row};
use uuid::Uuid;

use self::{
    entities::{Payment, User},
    error::DbError,
};

pub use tokio_postgres::types::Json;

#[derive(Deserialize, Debug, Clone)]
pub struct DbConfig {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

pub struct DbClient {
    inner: tokio_postgres::Client,
}

impl DbClient {
    pub async fn connect(db_config: DbConfig) -> Result<Self, DbError> {
        let (client, connection) = Config::new()
            .dbname(&db_config.name)
            .host(&db_config.host)
            .port(db_config.port)
            .user(&db_config.username)
            .password(db_config.password)
            .connect(NoTls)
            .await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        Ok(DbClient { inner: client })
    }

    pub async fn upsert_payment<T>(&self, payment: T, version: u32) -> Result<(), DbError>
    where
        T: Into<Payment>,
    {
        let payment = payment.into();
        let version: i32 = version.try_into().context("version overflow")?;
        let affected_rows = self
            .inner
            .execute(
                r#"
                INSERT INTO payments (
                    payment_id,
                    data_version,
                    created_at,
                    updated_at,
                    payment_data
                )
                VALUES($1, $2, NOW(), NOW(), $3)
                ON CONFLICT (payment_id) DO UPDATE SET
                    data_version = $2,
                    payment_data = $3,
                    updated_at = NOW()
                WHERE payments.data_version = $2 - 1
                "#,
                &[&payment.payment_id, &version, &payment.payment_data],
            )
            .await?;

        match affected_rows {
            0 => Err(DbError::ConcurrentUpdate),
            1 => Ok(()),
            n => Err(DbError::Unknown(anyhow::anyhow!(
                "More than one({}) row was updated",
                n
            ))),
        }
    }

    pub async fn get_payment<T>(
        &self,
        payment_id: impl AsRef<Uuid>,
    ) -> Result<Option<(T, u32)>, DbError>
    where
        T: From<Payment>,
    {
        let row = self
            .inner
            .query_opt(
                r#"
                SELECT
                    payment_id,
                    data_version,
                    payment_data
                FROM payments
                WHERE payment_id = $1
                "#,
                &[payment_id.as_ref()],
            )
            .await?;

        row.map(payment_from_row).transpose()
    }

    pub async fn get_payment_by_payout_id<T>(
        &self,
        payout_id: impl AsRef<Uuid>,
    ) -> Result<Option<(T, u32)>, DbError>
    where
        T: From<Payment>,
    {
        let row = self
            .inner
            .query_opt(
                r#"
                SELECT
                    p.payment_id,
                    p.data_version,
                    p.payment_data
                FROM 
                    payout_id_to_payment_id pid
                JOIN 
                    payments p ON pid.payment_id = p.payment_id
                WHERE 
                    pid.payout_id = $1
                "#,
                &[payout_id.as_ref()],
            )
            .await?;

        row.map(payment_from_row).transpose()
    }

    pub async fn register_payout_id(
        &self,
        payout_id: impl AsRef<Uuid>,
        payment_id: impl AsRef<Uuid>,
    ) -> Result<(), DbError> {
        self.inner
            .execute(
                r#"
                    INSERT INTO payments (
                        payout_id,
                        payment_id
                    )
                    VALUES($1, $2, NOW())
                "#,
                &[payout_id.as_ref(), payment_id.as_ref()],
            )
            .await?;
        Ok(())
    }

    pub async fn get_payment_by_refund_id<T>(
        &self,
        refund_id: impl AsRef<Uuid>,
    ) -> Result<Option<(T, u32)>, DbError>
    where
        T: From<Payment>,
    {
        let row = self
            .inner
            .query_opt(
                r#"
                SELECT
                    payment_id,
                    data_version,
                    payment_data
                FROM payments
                FROM 
                    refund_id_to_payment_id pid
                JOIN 
                    payments p ON pid.payment_id = p.payment_id
                WHERE 
                    pid.refund_id = $1
                "#,
                &[refund_id.as_ref()],
            )
            .await?;

        row.map(payment_from_row).transpose()
    }

    pub async fn get_payments<T>(&self, limit: i64, offset: i64) -> Result<Vec<(T, u32)>, DbError>
    where
        T: From<Payment>,
    {
        let rows = self
            .inner
            .query(
                r#"
                SELECT
                    payment_id,
                    data_version,
                    payment_data
                FROM payments
                LIMIT $1 
                OFFSET $2
                "#,
                &[&limit, &offset],
            )
            .await?;

        rows.into_iter()
            .map(payment_from_row)
            .collect::<Result<_, _>>()
    }

    pub async fn upsert_user<T>(&self, user: T, version: u32) -> Result<(), DbError>
    where
        T: Into<User>,
    {
        let user = user.into();
        let version: i32 = version.try_into().context("version overflow")?;
        let affected_rows = self
            .inner
            .execute(
                r#"
                INSERT INTO users (
                    user_id,
                    email,
                    data_version,
                    created_at,
                    updated_at,
                    user_data
                )
                VALUES($1, $2, $3, NOW(), NOW(), $4)
                ON CONFLICT (user_id) DO UPDATE SET
                    data_version = $3,
                    user_data = $4,
                    updated_at = NOW()
                WHERE users.data_version = $3 - 1
                "#,
                &[&user.user_id, &user.email, &version, &user.user_data],
            )
            .await?;

        match affected_rows {
            0 => Err(DbError::ConcurrentUpdate),
            1 => Ok(()),
            n => Err(DbError::Unknown(anyhow::anyhow!(
                "More than one({}) row was updated",
                n
            ))),
        }
    }

    pub async fn get_user<T>(&self, user_id: Uuid) -> Result<Option<(T, u32)>, DbError>
    where
        T: From<User>,
    {
        let row = self
            .inner
            .query_opt(
                r#"
                SELECT
                    user_id,
                    email,
                    data_version,
                    user_data
                FROM users
                WHERE user_id = $1
                "#,
                &[&user_id],
            )
            .await?;

        row.map(user_from_row).transpose()
    }

    pub async fn get_user_by_email<T>(&self, email: &str) -> Result<Option<(T, u32)>, DbError>
    where
        T: From<User>,
    {
        let row = self
            .inner
            .query_opt(
                r#"
                SELECT
                    user_id,
                    email,
                    data_version,
                    user_data
                FROM users
                WHERE email = $1
                "#,
                &[&email],
            )
            .await?;

        row.map(user_from_row).transpose()
    }

    pub async fn get_users<T>(&self, limit: i64, offset: i64) -> Result<Vec<(T, u32)>, DbError>
    where
        T: From<User>,
    {
        let rows = self
            .inner
            .query(
                r#"
                SELECT
                    user_id,
                    email,
                    data_version,
                    user_data
                FROM users
                LIMIT $1 
                OFFSET $2
                "#,
                &[&limit, &offset],
            )
            .await?;

        rows.into_iter()
            .map(user_from_row)
            .collect::<Result<_, _>>()
    }
}

fn user_from_row<T>(row: Row) -> Result<(T, u32), DbError>
where
    T: From<User>,
{
    let user = User {
        user_id: row.try_get(0)?,
        email: row.try_get(1)?,
        user_data: row.try_get(3)?,
    };
    let version: i32 = row.try_get(2)?;
    Ok((T::from(user), version as _))
}

fn payment_from_row<T>(row: Row) -> Result<(T, u32), DbError>
where
    T: From<Payment>,
{
    let payment = Payment {
        payment_id: row.try_get(0)?,
        payment_data: row.try_get(2)?,
    };
    let version: i32 = row.try_get(1)?;
    Ok((T::from(payment), version as _))
}
