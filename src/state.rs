use cosmwasm_std::Storage;

use crate::{Config, IntoResult, Result};

/// State is protected to ensure it is only accessed with predefined methods.
mod internal {
  use cw_storage_plus::Item;

  use super::*;

  /// Contract config state.
  pub const CONFIG: Item<Config> = Item::new("config");
}

/// Initialize contract state. For [`crate::entrypoint::instantiate`].
pub fn init_state(storage: &mut dyn Storage, config: &Config) -> Result {
  // init counters or other starting state here
  set_config(storage, config)?;
  Ok(())
}

/// Load contract config.
pub fn get_config(storage: &dyn Storage) -> Result<Config> {
  internal::CONFIG.load(storage).into_result()
}

/// Save contract config.
pub fn set_config(storage: &mut dyn Storage, config: &Config) -> Result {
  internal::CONFIG.save(storage, config)?;
  Ok(())
}

/// Update contract config.
///
/// Requires config to already exist.
pub fn update_config<A: FnOnce(Config) -> Result<Config>>(
  storage: &mut dyn Storage,
  action: A,
) -> Result {
  internal::CONFIG.update(storage, |prev| action(prev))?;
  Ok(())
}
