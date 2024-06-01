use std::fmt::Display;

use anyhow::Context;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

////////////////////////////////////////////////////////////////////////////////
// Common
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PaymentId(Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PayoutId(Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RefundId(Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct UserId(Uuid);

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
    pub payment_statuses: PaymentStatuses,
}

impl Payment {
    pub fn state(&self) -> PaymentState {
        self.payment_statuses.state()
    }
}

#[derive(Debug, Clone)]
pub struct PaymentStatuses {
    pub inbound_created_at: DateTime<Utc>,
    pub inbound_authorized_at: Option<DateTime<Utc>>,
    pub inbound_executed_at: Option<DateTime<Utc>>,
    pub inbound_settled_at: Option<DateTime<Utc>>,
    pub inbound_failed_at: Option<DateTime<Utc>>,
}

impl PaymentStatuses {
    pub fn state(&self) -> PaymentState {
        self.inbound_failed_at
            .map(|_| PaymentState::InboundFailed)
            .or_else(|| {
                self.inbound_settled_at
                    .map(|_| PaymentState::InboundSettled)
            })
            .or_else(|| {
                self.inbound_executed_at
                    .map(|_| PaymentState::InboundExecuted)
            })
            .or_else(|| {
                self.inbound_authorized_at
                    .map(|_| PaymentState::InboundAuthorized)
            })
            .unwrap_or(PaymentState::InboundCreated)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum PaymentState {
    // inbound status
    InboundCreated,
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
    OutboundFailed,
}

impl PaymentState {
    pub const fn as_str(self) -> &'static str {
        match self {
            PaymentState::InboundCreated => "inbound_created",
            PaymentState::InboundAuthorized => "inbound_authorized",
            PaymentState::InboundExecuted => "inbound_executed",
            PaymentState::InboundSettled => "inbound_settled",
            PaymentState::InboundFailed => "inbound_failed",
            PaymentState::RefundCreated => "refund_created",
            PaymentState::RefundAuthorized => "refund_authorized",
            PaymentState::RefundExecuted => "refund_executed",
            PaymentState::RefundFailed => "refund_failed",
            PaymentState::OutboundCreated => "outbound_created",
            PaymentState::OutboundAuthorized => "outbound_authorized",
            PaymentState::OutboundExecuted => "outbound_executed",
            PaymentState::OutboundFailed => "outbound_failed",
        }
    }
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
                payment_statuses,
            } => Payment {
                payment_id: PaymentId::from_uuid(value.payment_id),
                payer_full_name,
                payer_email,
                payee_full_name,
                payee_email,
                amount,
                security_question,
                security_answer,
                payment_statuses: PaymentStatuses::from_entity(payment_statuses),
            },
        }
    }
}

impl From<Payment> for db::entities::Payment {
    fn from(value: Payment) -> Self {
        db::entities::Payment {
            payment_id: value.payment_id.0,
            payment_data: db::Json(db::entities::PaymentData::V1 {
                payer_full_name: value.payer_full_name,
                payer_email: value.payer_email,
                payee_full_name: value.payee_full_name,
                payee_email: value.payee_email,
                amount: value.amount,
                security_question: value.security_question,
                security_answer: value.security_answer,
                payment_statuses: value.payment_statuses.into_entity(),
            }),
        }
    }
}

impl PaymentStatuses {
    const fn from_entity(value: db::entities::PaymentStatuses) -> Self {
        match value {
            db::entities::PaymentStatuses::V1(payment_status) => Self {
                inbound_created_at: payment_status.inbound_created_at,
                inbound_authorized_at: payment_status.inbound_authorized_at,
                inbound_executed_at: payment_status.inbound_executed_at,
                inbound_settled_at: payment_status.inbound_settled_at,
                inbound_failed_at: payment_status.inbound_failed_at,
            },
        }
    }

    const fn into_entity(self) -> db::entities::PaymentStatuses {
        db::entities::PaymentStatuses::V1(db::entities::PaymentStatusesV1 {
            inbound_created_at: self.inbound_created_at,
            inbound_authorized_at: self.inbound_authorized_at,
            inbound_executed_at: self.inbound_executed_at,
            inbound_settled_at: self.inbound_settled_at,
            inbound_failed_at: self.inbound_failed_at,
        })
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
            user_id: value.user_id.0,
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
                Self(Uuid::new_v4())
            }

            pub fn parse_str(uuid: &str) -> anyhow::Result<Self> {
                Uuid::parse_str(uuid)
                    .map(Self::from_uuid)
                    .context("Invalid Uuid")
            }

            pub const fn from_uuid(uuid: Uuid) -> Self {
                Self(uuid)
            }

            pub const fn into_uuid(self) -> Uuid {
                self.0
            }

            pub const fn as_uuid(&self) -> &Uuid {
                &self.0
            }
        }

        impl Display for $T {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl From<Uuid> for $T {
            fn from(value: Uuid) -> Self {
                Self(value)
            }
        }

        impl From<$T> for Uuid {
            fn from(value: $T) -> Self {
                value.0
            }
        }

        impl AsRef<Uuid> for $T {
            fn as_ref(&self) -> &Uuid {
                &self.0
            }
        }
    };
}

impl_uuid_ty!(PaymentId);
impl_uuid_ty!(PayoutId);
impl_uuid_ty!(RefundId);
impl_uuid_ty!(UserId);
