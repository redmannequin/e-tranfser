use actix_web::{http::header, web, HttpResponse};
use domain::{Payment, PaymentState};
use serde::Deserialize;
use uuid::Uuid;

use crate::AppContext;

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    code: String,
    #[serde(rename = "state")]
    payment_id: Uuid,
}

pub async fn tl_data_callback(
    app: web::Data<AppContext>,
    query_params: web::Query<QueryParams>,
) -> HttpResponse {
    let payment = app.db_client.get_payment(query_params.payment_id).await;

    match payment {
        Ok(Some((Payment { payment_state, .. }, _)))
            if (payment_state as u8) >= (PaymentState::InboundCreated as _) =>
        {
            HttpResponse::SeeOther()
                .insert_header((
                    header::LOCATION,
                    format!(
                        "/deposit_select_account?payment_id={}&code={}",
                        query_params.payment_id, query_params.code
                    ),
                ))
                .finish()
        }
        _ => HttpResponse::SeeOther()
            .insert_header((header::LOCATION, "/error"))
            .finish(),
    }
}
