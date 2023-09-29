mod entities;

use serde::Deserialize;
use tokio_postgres::{Config, NoTls};

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
                    security_question,
                    security_answer
                )
                SELECT $1, $2, $3, $4, $5
                "#,
                &[
                    &payment.payment_id,
                    &payment.full_name,
                    &payment.email,
                    &payment.security_question,
                    &payment.security_answer,
                ],
            )
            .await?;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error(transparent)]
    Postgres(#[from] tokio_postgres::Error),
}
