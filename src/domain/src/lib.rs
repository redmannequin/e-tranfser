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
// Payment Models
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
    pub payout_data: Option<PayoutData>,
    pub refund_data: Option<RefundData>,
}

impl Payment {
    pub fn state(&self) -> PaymentState {
        self.refund_data
            .as_ref()
            .map(RefundData::refund_state)
            .or_else(|| self.payout_data.as_ref().map(PayoutData::payout_state))
            .unwrap_or(self.payment_statuses.payment_state())
    }
}

#[derive(Debug, Clone)]
pub struct PayoutData {
    pub payout_id: PayoutId,
    pub payout_statuses: PayoutStatuses,
}

impl PayoutData {
    pub fn payout_state(&self) -> PaymentState {
        self.payout_statuses.payout_state()
    }
}

#[derive(Debug, Clone)]
pub struct PayoutStatuses {
    pub payout_created_at: DateTime<Utc>,
    pub payout_executed_at: Option<DateTime<Utc>>,
    pub payout_failed_at: Option<DateTime<Utc>>,
}

impl PayoutStatuses {
    pub fn payout_state(&self) -> PaymentState {
        self.payout_failed_at
            .map(|_| PaymentState::PayoutFailed)
            .or_else(|| {
                self.payout_executed_at
                    .map(|_| PaymentState::PayoutExecuted)
            })
            .unwrap_or(PaymentState::PayoutCreated)
    }
}

#[derive(Debug, Clone)]
pub struct RefundData {
    pub refund_id: RefundId,
    pub refund_statuses: RefundStatuses,
}

impl RefundData {
    pub fn refund_state(&self) -> PaymentState {
        self.refund_statuses.refund_state()
    }
}

#[derive(Debug, Clone)]
pub struct RefundStatuses {
    pub refund_created_at: DateTime<Utc>,
    pub refund_executed_at: Option<DateTime<Utc>>,
    pub refund_failed_at: Option<DateTime<Utc>>,
}

impl RefundStatuses {
    pub fn refund_state(&self) -> PaymentState {
        self.refund_failed_at
            .map(|_| PaymentState::RefundFailed)
            .or_else(|| {
                self.refund_executed_at
                    .map(|_| PaymentState::RefundExecuted)
            })
            .unwrap_or(PaymentState::RefundCreated)
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
    pub fn payment_state(&self) -> PaymentState {
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
    // outbound status
    PayoutCreated,
    PayoutExecuted,
    PayoutFailed,
    // refund status
    RefundCreated,
    RefundExecuted,
    RefundFailed,
}

impl PaymentState {
    pub const fn as_str(self) -> &'static str {
        match self {
            PaymentState::InboundCreated => "inbound_created",
            PaymentState::InboundAuthorized => "inbound_authorized",
            PaymentState::InboundExecuted => "inbound_executed",
            PaymentState::InboundSettled => "inbound_settled",
            PaymentState::InboundFailed => "inbound_failed",
            PaymentState::PayoutCreated => "payout_created",
            PaymentState::PayoutExecuted => "payout_executed",
            PaymentState::PayoutFailed => "payout_failed",
            PaymentState::RefundCreated => "refund_created",
            PaymentState::RefundExecuted => "refund_executed",
            PaymentState::RefundFailed => "refund_failed",
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// User Models
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub enum User {
    Registering {
        user_id: UserId,
        email: String,
        first_name: String,
        last_name: String,
        code: String,
        timestamp: DateTime<Utc>,
    },
    Registered {
        user_id: UserId,
        email: String,
        first_name: String,
        last_name: String,
    },
}

impl User {
    pub fn state(&self) -> UserState {
        match self {
            User::Registered { .. } => UserState::Registered,
            User::Registering { .. } => UserState::Registering,
        }
    }

    pub fn user_id(&self) -> UserId {
        match self {
            User::Registered { user_id, .. } => *user_id,
            User::Registering { user_id, .. } => *user_id,
        }
    }

    pub fn email(&self) -> &str {
        match self {
            User::Registered { email, .. } => email,
            User::Registering { email, .. } => email,
        }
    }

    pub fn first_name(&self) -> &str {
        match self {
            User::Registered { first_name, .. } => first_name,
            User::Registering { first_name, .. } => first_name,
        }
    }

    pub fn last_name(&self) -> &str {
        match self {
            User::Registered { last_name, .. } => last_name,
            User::Registering { last_name, .. } => last_name,
        }
    }

    pub fn registration_code(&self) -> Option<(&str, &DateTime<Utc>)> {
        match self {
            User::Registered { .. } => None,
            User::Registering {
                timestamp, code, ..
            } => Some((code, timestamp)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserState {
    Registering,
    Registered,
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
                payout_data,
                refund_data,
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
                payout_data: payout_data.map(PayoutData::from_entity),
                refund_data: refund_data.map(RefundData::from_entity),
            },
        }
    }
}

impl PayoutData {
    pub const fn from_entity(value: db::entities::PayoutData) -> Self {
        PayoutData {
            payout_id: PayoutId::from_uuid(value.payout_id),
            payout_statuses: PayoutStatuses {
                payout_created_at: value.payout_statuses.payout_created_at,
                payout_executed_at: value.payout_statuses.payout_executed_at,
                payout_failed_at: value.payout_statuses.payout_failed_at,
            },
        }
    }

    pub const fn to_entity(self) -> db::entities::PayoutData {
        db::entities::PayoutData {
            payout_id: self.payout_id.into_uuid(),
            payout_statuses: db::entities::PayoutStatuses {
                payout_created_at: self.payout_statuses.payout_created_at,
                payout_executed_at: self.payout_statuses.payout_executed_at,
                payout_failed_at: self.payout_statuses.payout_failed_at,
            },
        }
    }
}

impl From<PayoutData> for db::entities::PayoutData {
    fn from(value: PayoutData) -> Self {
        db::entities::PayoutData {
            payout_id: value.payout_id.into_uuid(),
            payout_statuses: db::entities::PayoutStatuses {
                payout_created_at: value.payout_statuses.payout_created_at,
                payout_executed_at: value.payout_statuses.payout_executed_at,
                payout_failed_at: value.payout_statuses.payout_failed_at,
            },
        }
    }
}

impl RefundData {
    pub const fn from_entity(value: db::entities::RefundData) -> Self {
        RefundData {
            refund_id: RefundId::from_uuid(value.refund_id),
            refund_statuses: RefundStatuses {
                refund_created_at: value.refund_statuses.refund_created_at,
                refund_executed_at: value.refund_statuses.refund_executed_at,
                refund_failed_at: value.refund_statuses.refund_failed_at,
            },
        }
    }

    pub const fn to_entity(self) -> db::entities::RefundData {
        db::entities::RefundData {
            refund_id: self.refund_id.into_uuid(),
            refund_statuses: db::entities::RefundStatuses {
                refund_created_at: self.refund_statuses.refund_created_at,
                refund_executed_at: self.refund_statuses.refund_executed_at,
                refund_failed_at: self.refund_statuses.refund_failed_at,
            },
        }
    }
}

impl From<RefundData> for db::entities::RefundData {
    fn from(value: RefundData) -> Self {
        db::entities::RefundData {
            refund_id: value.refund_id.into_uuid(),
            refund_statuses: db::entities::RefundStatuses {
                refund_created_at: value.refund_statuses.refund_created_at,
                refund_executed_at: value.refund_statuses.refund_executed_at,
                refund_failed_at: value.refund_statuses.refund_failed_at,
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
                payout_data: value.payout_data.map(PayoutData::to_entity),
                refund_data: value.refund_data.map(RefundData::to_entity),
            }),
        }
    }
}

impl PaymentStatuses {
    const fn from_entity(value: db::entities::PaymentStatuses) -> Self {
        Self {
            inbound_created_at: value.inbound_created_at,
            inbound_authorized_at: value.inbound_authorized_at,
            inbound_executed_at: value.inbound_executed_at,
            inbound_settled_at: value.inbound_settled_at,
            inbound_failed_at: value.inbound_failed_at,
        }
    }

    const fn into_entity(self) -> db::entities::PaymentStatuses {
        db::entities::PaymentStatuses {
            inbound_created_at: self.inbound_created_at,
            inbound_authorized_at: self.inbound_authorized_at,
            inbound_executed_at: self.inbound_executed_at,
            inbound_settled_at: self.inbound_settled_at,
            inbound_failed_at: self.inbound_failed_at,
        }
    }
}

impl From<db::entities::User> for User {
    fn from(value: db::entities::User) -> Self {
        match value.user_data.0 {
            db::entities::UserData::V1(db::entities::v1::UserDataV1::Registered {
                first_name,
                last_name,
            }) => User::Registered {
                user_id: UserId::from(value.user_id),
                email: value.email,
                first_name,
                last_name,
            },
            db::entities::UserData::V1(db::entities::v1::UserDataV1::Registering {
                first_name,
                last_name,
                code,
                timestamp,
            }) => User::Registering {
                user_id: UserId::from(value.user_id),
                email: value.email,
                first_name,
                last_name,
                code,
                timestamp,
            },
        }
    }
}

impl From<User> for db::entities::User {
    fn from(value: User) -> Self {
        match value {
            User::Registering {
                user_id,
                email,
                first_name,
                last_name,
                code,
                timestamp,
            } => db::entities::User {
                user_id: user_id.into_uuid(),
                email,
                user_data: db::Json(db::entities::UserData::V1(
                    db::entities::v1::UserDataV1::Registering {
                        first_name,
                        last_name,
                        code,
                        timestamp,
                    },
                )),
            },
            User::Registered {
                user_id,
                email,
                first_name,
                last_name,
            } => db::entities::User {
                user_id: user_id.into_uuid(),
                email,
                user_data: db::Json(db::entities::UserData::V1(
                    db::entities::v1::UserDataV1::Registered {
                        first_name,
                        last_name,
                    },
                )),
            },
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
