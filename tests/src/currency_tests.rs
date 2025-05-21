use std::sync::Mutex;

use currency::{
    state::TransactionState,
    types::{canister_wallet::CanisterWalletMetadata, currency_manager::CurrencyManager},
};

struct WalletMeta {
    transaction_state: Mutex<TransactionState>,
}

impl WalletMeta {
    fn new() -> Self {
        Self {
            transaction_state: Mutex::new(TransactionState::new()),
        }
    }
}

impl CanisterWalletMetadata for WalletMeta {
    fn get_transaction_state(&self) -> &Mutex<TransactionState> {
        &self.transaction_state
    }

    fn handle_cycle_check(&self) {}
}

#[test]
fn can_create_currency_manager() {
    let meta = WalletMeta::new();
    CurrencyManager::new(meta);
}

#[test]
fn can_get_transaction_state() {
    let meta = WalletMeta::new();
    let transaction_state = meta.get_transaction_state();
    assert!(transaction_state.lock().is_ok());
}
