use actix_web::{http::header, web, HttpResponse};
use domain::{Payment, PaymentState};
use serde::Deserialize;
use uuid::Uuid;

use crate::{app::deposit_flow::DESPOSIT_SELECT_ACCOUNT_PAGE, AppContext};

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
    let payment = app
        .db_client
        .get_payment::<Payment>(query_params.payment_id)
        .await;

    match payment {
        Ok(Some((payment, _))) if payment.state() >= PaymentState::InboundCreated => {
            HttpResponse::SeeOther()
                .insert_header((
                    header::LOCATION,
                    format!(
                        "{}?payment_id={}&code={}",
                        DESPOSIT_SELECT_ACCOUNT_PAGE, query_params.payment_id, query_params.code
                    ),
                ))
                .finish()
        }
        _ => HttpResponse::SeeOther()
            .insert_header((header::LOCATION, "/error"))
            .finish(),
    }
}
