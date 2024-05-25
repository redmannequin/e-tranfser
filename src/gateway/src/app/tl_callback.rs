use actix_web::{http::header, web, HttpResponse};
use domain::{Payment, PaymentState};
use serde::Deserialize;
use uuid::Uuid;

use crate::AppContext;

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    payment_id: Uuid,
}

pub async fn tl_callback(
    app: web::Data<AppContext>,
    query_params: web::Query<QueryParams>,
) -> HttpResponse {
    tracing::info!("payment sent: {}", query_params.payment_id);

    let payment = app
        .db_client
        .get_payment::<Payment>(query_params.payment_id)
        .await;

    match payment {
        Ok(Some((
            Payment {
                payment_statuses, ..
            },
            _version,
        ))) if payment_statuses.state() >= PaymentState::InboundCreated => HttpResponse::SeeOther()
            .insert_header((
                header::LOCATION,
                format!("/payment_sent?payment_id={}", query_params.payment_id),
            ))
            .finish(),
        _ => HttpResponse::SeeOther()
            .insert_header((header::LOCATION, "/error"))
            .finish(),
    }
}
