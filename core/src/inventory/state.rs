use crate::error::CoreResult;
use crate::inventory::kind::ItemKind;
use crate::inventory::state::quantity::ItemQuantity;
use sea_orm::prelude::Json;
use std::fmt::Debug;

mod quantity;

pub trait ItemComponent:
    Debug + Default + Clone + serde::Serialize + for<'de> serde::Deserialize<'de>
{
    fn sanitize(state: &mut Option<Self>, kind: ItemKind);
    fn default_for(kind: ItemKind) -> Option<Self>;
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ItemState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<ItemQuantity>,
}

impl ItemState {
    pub fn default_for(kind: ItemKind) -> Self {
        Self {
            quantity: ItemQuantity::default_for(kind),
        }
    }

    pub fn sanitize(&mut self, kind: ItemKind) {
        ItemQuantity::sanitize(&mut self.quantity, kind);
    }

    pub fn deserialize(json: &Json) -> CoreResult<Self> {
        Ok(serde_json::from_value(json.clone())?)
    }

    pub fn serialize(&self) -> CoreResult<Json> {
        Ok(serde_json::to_value(self)?)
    }

    pub fn add(&mut self, count: u64) {
        if let Some(quantity) = &mut self.quantity {
            quantity.add(count);
        }
    }

    pub fn with_count(mut self, count: u64) -> Self {
        if let Some(quantity) = &mut self.quantity {
            quantity.count = count;
        }
        self
    }
}
