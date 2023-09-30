use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use uuid::Uuid;

#[derive(Debug)]
pub struct TlClient {
    pub client_id: String,
    pub redirect_url: String,
    pub merchant_accounts: Vec<MerchantAccount>,
}

#[derive(Debug)]
pub struct MerchantAccount {
    pub merchant_account_id: Uuid,
    pub currency: String,
    pub balance: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug)]
pub enum Transaction {
    Inbound { payment_id: Uuid, amount: u64 },
    Outbound { payout_id: Uuid, amount: u64 },
}

#[derive(Debug, Clone)]
pub struct State {
    inner: Arc<Mutex<HashMap<String, TlClient>>>,
}

impl State {
    pub fn new() -> Self {
        State {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create_client(&self, client_id: String, redirect_url: String) {
        let mut state = self.inner.lock().unwrap();
        state.insert(
            client_id.clone(),
            TlClient {
                client_id,
                redirect_url,
                merchant_accounts: vec![],
            },
        );
    }

    pub fn add_merchant_account(
        &self,
        client_id: &str,
        merchant_account_id: Uuid,
        currency: String,
        starting_balance: u64,
    ) {
        let mut state = self.inner.lock().unwrap();
        state.get_mut(client_id).map(|tl_client| {
            tl_client.merchant_accounts.push(MerchantAccount {
                merchant_account_id,
                currency,
                balance: starting_balance,
                transactions: vec![],
            })
        });
    }

    pub fn get_balance(&self, client_id: &str, merchant_account_id: Uuid) -> Option<u64> {
        let state = self.inner.lock().unwrap();
        state
            .get(client_id)
            .and_then(|tl_client| {
                tl_client
                    .merchant_accounts
                    .iter()
                    .find(|ma| ma.merchant_account_id == merchant_account_id)
            })
            .map(|ma| ma.balance)
    }

    pub fn create_ma_payment(
        &self,
        client_id: &str,
        merchant_account_id: Uuid,
        currency: &str,
        amount: u64,
    ) -> Option<Uuid> {
        let mut state = self.inner.lock().unwrap();
        state.get_mut(client_id).and_then(|tl_client| {
            tl_client
                .merchant_accounts
                .iter_mut()
                .find(|ma| ma.merchant_account_id == merchant_account_id && ma.currency == currency)
                .map(|ma| {
                    let payment_id = Uuid::new_v4();
                    ma.balance += amount;
                    ma.transactions
                        .push(Transaction::Inbound { payment_id, amount });
                    payment_id
                })
        })
    }

    pub fn create_payout(
        &self,
        client_id: &str,
        merchant_account_id: Uuid,
        amount: u64,
    ) -> Option<Uuid> {
        let mut state = self.inner.lock().unwrap();
        state.get_mut(client_id).and_then(|tl_client| {
            tl_client
                .merchant_accounts
                .iter_mut()
                .find(|ma| ma.merchant_account_id == merchant_account_id)
                .and_then(|ma| {
                    if ma.balance >= amount {
                        let payout_id = Uuid::new_v4();
                        ma.balance -= amount;
                        ma.transactions
                            .push(Transaction::Outbound { payout_id, amount });
                        Some(payout_id)
                    } else {
                        None
                    }
                })
        })
    }
}
