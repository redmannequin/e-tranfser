use chrono::Utc;
use domain::{Payment, PaymentId, PaymentState, PayoutData, PayoutId, PayoutStatuses};

use crate::AppContext;

pub async fn create_payout(
    app: &AppContext,
    payment_id: PaymentId,
    iban: &str,
    reference: &str,
) -> PayoutId {
    let (mut payment, version) = app
        .db_client
        .get_payment::<Payment>(payment_id)
        .await
        .unwrap()
        .unwrap();

    if payment.state() >= PaymentState::PayoutCreated {
        todo!()
    }

    payment.payout_data = Some(PayoutData::PayoutRegistered {
        payout_registered_at: Utc::now(),
    });

    app.db_client
        .upsert_payment(payment.clone(), version + 1)
        .await
        .unwrap();

    // TODO: publish to message queue here to return faster; by moving the
    //       truelayer and database call outside and only needing to register
    //       the payout.
    let payout = app
        .tl_client
        .create_payout(&payment.payee_full_name, iban, payment.amount, reference)
        .await
        .unwrap();

    let payout_id = PayoutId::from_uuid(payout.payout_id);

    payment.payout_data = Some(PayoutData::PayoutCreated {
        payout_id,
        payout_statuses: PayoutStatuses {
            payout_created_at: Utc::now(),
            payout_executed_at: None,
            payout_failed_at: None,
        },
    });

    app.db_client
        .upsert_payment(payment, version + 1)
        .await
        .unwrap();

    payout_id
}
