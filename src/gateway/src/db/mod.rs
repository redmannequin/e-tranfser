mod entities;

use serde::Deserialize;
use tokio_postgres::{Config, NoTls};
use uuid::Uuid;

pub use self::entities::CreatePayment;

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

    pub async fn insert_payment(&self, payment: CreatePayment) -> Result<(), DbError> {
        self.inner
            .execute(
                r#"
                INSERT INTO payments (
                    payment_id,
                    full_name,
                    email,
                    amount,
                    security_question,
                    security_answer
                )
                SELECT $1, $2, $3, $4, $5, $6
                "#,
                &[
                    &payment.payment_id,
                    &payment.full_name,
                    &payment.email,
                    &(payment.amount as i32),
                    &payment.security_question,
                    &payment.security_answer,
                ],
            )
            .await?;
        Ok(())
    }

    pub async fn get_payment(&self, payment_id: Uuid) -> Result<CreatePayment, DbError> {
        Ok(self
            .inner
            .query_one(
                r#"
                    SELECT payment_id, full_name, email, amount, security_question, security_answer
                    From payments
                    WHERE payment_id = $1
                "#,
                &[&payment_id],
            )
            .await
            .map(|row| CreatePayment {
                payment_id: row.get(0),
                full_name: row.get(1),
                email: row.get(2),
                amount: row.get::<_, i32>(3) as _,
                security_question: row.get(4),
                security_answer: row.get(5),
            })?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error(transparent)]
    Postgres(#[from] tokio_postgres::Error),
}
