use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreatePayment {
    pub payment_id: Uuid,
    pub payer_full_name: String,
    pub payer_email: String,
    pub payee_full_name: String,
    pub payee_email: String,
    pub amount: u32,
    pub security_question: String,
    pub security_answer: String,
    pub state: PaymentState,
}

#[derive(Debug, Clone)]
pub struct User {
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub primary_account_id: Option<Uuid>,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum PaymentState {
    // inbound status 1..19
    InboundCreated = 1,
    InboundAuthorizing = 2,
    InboundAuthorized = 3,
    InboundExecuted = 4,
    InboundSettled = 5,
    InboundFailed = 6,
    // refund status 20..39
    RefundCreated = 20,
    RefundAuthorized = 21,
    RefundExecuted = 22,
    RefundFailed = 23,
    // outbound status 40..
    OutboundCreated = 40,
    OutboundAuthorized = 41,
    OutboundExecuted = 42,
    OutboundFaild = 43,
}

impl From<u8> for PaymentState {
    fn from(value: u8) -> Self {
        match value {
            1 => PaymentState::InboundCreated,
            2 => PaymentState::InboundAuthorizing,
            3 => PaymentState::InboundAuthorized,
            4 => PaymentState::InboundExecuted,
            5 => PaymentState::InboundSettled,
            6 => PaymentState::InboundFailed,
            20 => PaymentState::RefundCreated,
            21 => PaymentState::RefundAuthorized,
            22 => PaymentState::RefundExecuted,
            23 => PaymentState::RefundFailed,
            40 => PaymentState::OutboundCreated,
            41 => PaymentState::OutboundAuthorized,
            42 => PaymentState::OutboundExecuted,
            43 => PaymentState::OutboundFaild,
            _ => panic!("DB ERROR INVALID PAYMENT_STATE"),
        }
    }
}
