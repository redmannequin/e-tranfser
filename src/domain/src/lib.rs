use std::fmt::Display;

use anyhow::Context;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

////////////////////////////////////////////////////////////////////////////////
// Common
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PaymentId {
    inner: Uuid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct UserId {
    inner: Uuid,
}

////////////////////////////////////////////////////////////////////////////////
// Models
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct Payment {
    pub payment_id: PaymentId,
    pub payer_full_name: String,
    pub payer_email: String,
    pub payee_full_name: String,
    pub payee_email: String,
    pub amount: u32,
    pub security_question: String,
    pub security_answer: String,
    pub payment_state: PaymentState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PaymentState {
    // inbound status
    InboundCreated,
    InboundAuthorizing,
    InboundAuthorized,
    InboundExecuted,
    InboundSettled,
    InboundFailed,
    // refund status
    RefundCreated,
    RefundAuthorized,
    RefundExecuted,
    RefundFailed,
    // outbound status
    OutboundCreated,
    OutboundAuthorized,
    OutboundExecuted,
    OutboundFaild,
}

#[derive(Debug, Clone)]
pub struct User {
    pub user_id: UserId,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

////////////////////////////////////////////////////////////////////////////////
// Database Mappings
////////////////////////////////////////////////////////////////////////////////

impl From<db::entities::Payment> for Payment {
    fn from(value: db::entities::Payment) -> Self {
        match value.payment_data.0 {
            db::entities::PaymentData::V1 {
                payer_full_name,
                payer_email,
                payee_full_name,
                payee_email,
                amount,
                security_question,
                security_answer,
                payment_state,
            } => Payment {
                payment_id: PaymentId::from_uuid(value.payment_id),
                payer_full_name,
                payer_email,
                payee_full_name,
                payee_email,
                amount,
                security_question,
                security_answer,
                payment_state: PaymentState::from_entity(payment_state),
            },
        }
    }
}

impl From<Payment> for db::entities::Payment {
    fn from(value: Payment) -> Self {
        db::entities::Payment {
            payment_id: value.payment_id.inner,
            payment_data: db::Json(db::entities::PaymentData::V1 {
                payer_full_name: value.payer_full_name,
                payer_email: value.payer_email,
                payee_full_name: value.payee_full_name,
                payee_email: value.payee_email,
                amount: value.amount,
                security_question: value.security_question,
                security_answer: value.security_answer,
                payment_state: value.payment_state.into_entity(),
            }),
        }
    }
}

impl PaymentState {
    const fn from_entity(value: db::entities::PaymentState) -> Self {
        match value {
            db::entities::PaymentState::InboundCreated => PaymentState::InboundCreated,
            db::entities::PaymentState::InboundAuthorizing => PaymentState::InboundAuthorizing,
            db::entities::PaymentState::InboundAuthorized => PaymentState::InboundAuthorized,
            db::entities::PaymentState::InboundExecuted => PaymentState::InboundExecuted,
            db::entities::PaymentState::InboundSettled => PaymentState::InboundSettled,
            db::entities::PaymentState::InboundFailed => PaymentState::InboundFailed,
            db::entities::PaymentState::RefundCreated => PaymentState::RefundCreated,
            db::entities::PaymentState::RefundAuthorized => PaymentState::RefundAuthorized,
            db::entities::PaymentState::RefundExecuted => PaymentState::RefundExecuted,
            db::entities::PaymentState::RefundFailed => PaymentState::RefundFailed,
            db::entities::PaymentState::OutboundCreated => PaymentState::OutboundCreated,
            db::entities::PaymentState::OutboundAuthorized => PaymentState::OutboundAuthorized,
            db::entities::PaymentState::OutboundExecuted => PaymentState::OutboundExecuted,
            db::entities::PaymentState::OutboundFaild => PaymentState::OutboundFaild,
        }
    }

    const fn into_entity(self) -> db::entities::PaymentState {
        match self {
            PaymentState::InboundCreated => db::entities::PaymentState::InboundCreated,
            PaymentState::InboundAuthorizing => db::entities::PaymentState::InboundAuthorizing,
            PaymentState::InboundAuthorized => db::entities::PaymentState::InboundAuthorized,
            PaymentState::InboundExecuted => db::entities::PaymentState::InboundExecuted,
            PaymentState::InboundSettled => db::entities::PaymentState::InboundSettled,
            PaymentState::InboundFailed => db::entities::PaymentState::InboundFailed,
            PaymentState::RefundCreated => db::entities::PaymentState::RefundCreated,
            PaymentState::RefundAuthorized => db::entities::PaymentState::RefundAuthorized,
            PaymentState::RefundExecuted => db::entities::PaymentState::RefundExecuted,
            PaymentState::RefundFailed => db::entities::PaymentState::RefundFailed,
            PaymentState::OutboundCreated => db::entities::PaymentState::OutboundCreated,
            PaymentState::OutboundAuthorized => db::entities::PaymentState::OutboundAuthorized,
            PaymentState::OutboundExecuted => db::entities::PaymentState::OutboundExecuted,
            PaymentState::OutboundFaild => db::entities::PaymentState::OutboundFaild,
        }
    }
}

impl From<db::entities::User> for User {
    fn from(value: db::entities::User) -> Self {
        match value.user_data.0 {
            db::entities::UserData::V1 {
                first_name,
                last_name,
            } => User {
                user_id: UserId::from(value.user_id),
                email: value.email,
                first_name,
                last_name,
            },
        }
    }
}

impl From<User> for db::entities::User {
    fn from(value: User) -> Self {
        db::entities::User {
            user_id: value.user_id.inner,
            email: value.email,
            user_data: db::Json(db::entities::UserData::V1 {
                first_name: value.first_name,
                last_name: value.last_name,
            }),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Macros
////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_uuid_ty {
    ($T:ty) => {
        impl $T {
            #[allow(clippy::new_without_default)]
            pub fn new() -> Self {
                Self {
                    inner: Uuid::new_v4(),
                }
            }

            pub fn parse_str(uuid: &str) -> anyhow::Result<Self> {
                Uuid::parse_str(uuid)
                    .map(Self::from_uuid)
                    .context("Invalid Uuid")
            }

            pub const fn from_uuid(uuid: Uuid) -> Self {
                Self { inner: uuid }
            }

            pub const fn into_uuid(self) -> Uuid {
                self.inner
            }

            pub const fn as_uuid(&self) -> &Uuid {
                &self.inner
            }
        }

        impl Display for $T {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.inner.fmt(f)
            }
        }

        impl From<Uuid> for $T {
            fn from(value: Uuid) -> Self {
                Self { inner: value }
            }
        }

        impl From<$T> for Uuid {
            fn from(value: $T) -> Self {
                value.inner
            }
        }
    };
}

impl_uuid_ty!(PaymentId);
impl_uuid_ty!(UserId);
