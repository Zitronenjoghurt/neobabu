use crate::database::entity::user;
use crate::error::CoreResult;
use crate::stores::economy::Balance;
use crate::stores::Stores;
use crate::types::currency::Currency;
use std::collections::HashMap;
use std::sync::Arc;
use strum::IntoEnumIterator;

pub struct EconomyService {
    stores: Arc<Stores>,
}

pub struct Wallet {
    pub currencies: HashMap<Currency, Balance>,
}

impl EconomyService {
    pub fn initialize(stores: &Arc<Stores>) -> Arc<Self> {
        Arc::new(Self {
            stores: stores.clone(),
        })
    }

    pub async fn wallet(&self, user: &user::Model) -> CoreResult<Wallet> {
        let mut currencies = HashMap::new();

        for currency in Currency::iter() {
            let balance = self.stores.economy.balance(user, currency).await?;
            currencies.insert(currency, balance);
        }

        Ok(Wallet { currencies })
    }
}
