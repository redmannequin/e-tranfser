use actix_web::{http::header, web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

use crate::{db::CreatePayment, AppContext};

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    payment_id: Uuid,
}

pub async fn tl_callback(
    app: web::Data<AppContext>,
    query_params: web::Query<QueryParams>,
) -> HttpResponse {
    tracing::info!("payment sent: {}", query_params.payment_id);

    let payment = app.db_client.get_payment(query_params.payment_id).await;

    match payment {
        Ok(CreatePayment {
            deposited: false, ..
        }) => HttpResponse::SeeOther()
            .insert_header((header::LOCATION, "/payment_sent"))
            .finish(),
        _ => HttpResponse::SeeOther()
            .insert_header((header::LOCATION, "/error"))
            .finish(),
    }
}
