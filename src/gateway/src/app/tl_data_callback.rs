use actix_web::{http::header, web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    db::{CreatePayment, PaymentState},
    AppContext,
};

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
        Ok(CreatePayment { state, .. }) if (state as u8) >= (PaymentState::InboundCreated as _) => {
            HttpResponse::SeeOther()
                .insert_header((
                    header::LOCATION,
                    format!(
                        "/deposit_select_account?payment_id={}",
                        query_params.payment_id
                    ),
                ))
                .finish()
        }
        _ => HttpResponse::SeeOther()
            .insert_header((header::LOCATION, "/error"))
            .finish(),
    }
}
