use chrono::Utc;
use domain::{Payment, PaymentId, PaymentStatuses};

use crate::AppContext;

pub struct PaymentInfo<'a> {
    pub payer_full_name: &'a str,
    pub payer_email: &'a str,
    pub payee_full_name: &'a str,
    pub payee_email: &'a str,
    pub amount: u32,
    pub security_question: &'a str,
    pub security_answer: &'a str,
}

pub async fn create_payment<'a>(app: &AppContext, payment_info: PaymentInfo<'a>) -> PaymentId {
    let payment = app
        .tl_client
        .create_ma_payment(
            payment_info.payer_full_name,
            payment_info.payer_email,
            None,
            payment_info.amount,
            "test",
        )
        .await
        .unwrap();

    let payment_id = PaymentId::from_uuid(payment.payment_id);

    app.db_client
        .upsert_payment(
            Payment {
                payment_id,
                payer_full_name: payment_info.payer_full_name.to_string(),
                payer_email: payment_info.payer_email.to_string(),
                payee_full_name: payment_info.payee_full_name.to_string(),
                payee_email: payment_info.payee_email.to_string(),
                amount: payment_info.amount,
                security_question: payment_info.security_question.to_string(),
                security_answer: payment_info.security_answer.to_string(),
                payment_statuses: PaymentStatuses {
                    inbound_created_at: Utc::now(),
                    inbound_authorized_at: None,
                    inbound_executed_at: None,
                    inbound_settled_at: None,
                    inbound_failed_at: None,
                },
                payout_data: None,
                refund_data: None,
            },
            0,
        )
        .await
        .unwrap();

    payment_id
}
