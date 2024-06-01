use actix_web::{http::header, web, HttpResponse};
use domain::Payment;
use serde::Deserialize;
use uuid::Uuid;

use crate::{app::deposit_flow::DESPOSIT_STATUS_PAGE, AppContext};

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    payment_id: Uuid,
}

pub async fn tl_deposit_callback(
    app: web::Data<AppContext>,
    query_params: web::Query<QueryParams>,
) -> HttpResponse {
    let payment = app
        .db_client
        .get_payment::<Payment>(query_params.payment_id)
        .await;

    match payment {
        Ok(Some(_)) => HttpResponse::SeeOther()
            .insert_header((
                header::LOCATION,
                format!(
                    "{}?payment_id={}",
                    DESPOSIT_STATUS_PAGE, query_params.payment_id
                ),
            ))
            .finish(),
        _ => HttpResponse::SeeOther()
            .insert_header((header::LOCATION, "/error"))
            .finish(),
    }
}
