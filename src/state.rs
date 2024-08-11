use cw_storage_plus::Item;

use crate::cfg::Cfg;

/// Contract config state.
pub const CFG: Item<Cfg> = Item::new("cfg");
