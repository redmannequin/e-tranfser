mod entities;

use serde::Deserialize;
use tokio_postgres::{Config, NoTls};
use uuid::Uuid;

pub use self::entities::{CreatePayment, PaymentState, User};

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
                    payer_full_name,
                    payer_email,
                    payee_full_name,
                    payee_email,
                    amount,
                    security_question,
                    security_answer,
                    state
                )
                SELECT $1, $2, $3, $4, $5, $6, $7, $8, $9
                "#,
                &[
                    &payment.payment_id,
                    &payment.payer_full_name,
                    &payment.payer_email,
                    &payment.payee_full_name,
                    &payment.payee_email,
                    &(payment.amount as i32),
                    &payment.security_question,
                    &payment.security_answer,
                    &(PaymentState::InboundCreated as i16)
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
                    SELECT payment_id, payer_full_name, payer_email, payee_full_name, payee_email, amount, security_question, security_answer, state
                    From payments
                    WHERE payment_id = $1
                "#,
                &[&payment_id],
            )
            .await
            .map(|row| CreatePayment {
                payment_id: row.get(0),
                payer_full_name: row.get(1),
                payer_email: row.get(2),
                payee_full_name: row.get(3),
                payee_email: row.get(4),
                amount: row.get::<_, i32>(5) as _,
                security_question: row.get(6),
                security_answer: row.get(7),
                state: PaymentState::from(row.get::<_, i16>(8) as u8)             
            })?)
    }

    pub async fn set_payment_state(&self, payment_id: Uuid, payment_state: PaymentState) -> Result<(), DbError> {
        self.inner
            .execute(
                r#"
                    UPDATE payments 
                    SET state = $2 
                    WHERE payment_id = $1
                "#,
                &[&payment_id, &(payment_state as i16)],
            )
            .await?;
        Ok(())
    }


    pub async fn insert_user(&self, user: User) -> Result<(), DbError> {
        self.inner.execute(
            r#"
            INSERT INTO users (
                first_name,
                last_name,
                email,
                primary_account_id
            )
            SELECT $1, $2, $3, $4
            "#, &[
                &user.first_name,
                &user.last_name,
                &user.email,
                &Option::<Uuid>::None
            ]
        ).await?;
            Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error(transparent)]
    Postgres(#[from] tokio_postgres::Error),
}
