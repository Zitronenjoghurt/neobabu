use crate::inventory::kind::ItemKind;
use crate::inventory::state::ItemComponent;

#[derive(Debug, Default, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct ItemQuantity {
    pub count: u64,
}

impl ItemComponent for ItemQuantity {
    fn sanitize(state: &mut Option<Self>, kind: ItemKind) {
        if !Self::is_stackable(kind) {
            *state = None;
        } else if state.is_none() {
            *state = Some(Self::default());
        }

        // ToDo: Inventory actions for stack size
    }

    fn default_for(kind: ItemKind) -> Option<Self> {
        if Self::is_stackable(kind) {
            Some(Self::default())
        } else {
            None
        }
    }
}

impl ItemQuantity {
    pub fn add(&mut self, count: u64) {
        self.count = self.count.saturating_add(count);
    }
}

// Definition
impl ItemQuantity {
    pub fn is_stackable(kind: ItemKind) -> bool {
        match kind {
            ItemKind::Carrot => true,
        }
    }

    pub fn stack_size(kind: ItemKind) -> Option<u64> {
        match kind {
            ItemKind::Carrot => None,
        }
    }
}
