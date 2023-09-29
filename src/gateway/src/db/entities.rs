use uuid::Uuid;

pub struct CreatePayment {
    pub payment_id: Uuid,
    pub full_name: String,
    pub email: String,
    pub amount: u64,
    pub security_question: String,
    pub security_answer: String,
}
