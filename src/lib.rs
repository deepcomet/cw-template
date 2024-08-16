mod config;
mod contract;
mod data;
mod error;
mod exec;
mod query;
mod state;
pub mod util;

pub use crate::config::{Config, ConfigMsg};
pub use crate::contract::{% raw %}{{% endraw %}{{project-name-pascal}}{% raw %}}{% endraw %};
// pub use crate::data::
pub use crate::error::{Error, IntoResult, Result};
pub use crate::exec::ExecMsg;
pub use crate::query::QueryMsg;
pub type InstantiateMsg = ConfigMsg;

pub mod entrypoint {
  #[cfg(not(feature = "library"))]
  use cosmwasm_std::entry_point;
  use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
  use cw2::set_contract_version;

  use super::*;
  use crate::state::init_state;

  /// Contract name (cargo package name).
  const CONTRACT_NAME: &str = "crates.io:{{project-name}}";
  /// Contract version (cargo package version).
  const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

  /// Contract instantiate entrypoint.
  #[cfg_attr(not(feature = "library"), entry_point)]
  pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
  ) -> Result<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    init_state(deps.storage, &msg.validate(deps.api)?)?;
    Ok(
      Response::default()
        .add_attribute("method", "instantiate")
        .add_attribute("sender", info.sender.to_string())
        .add_attribute("name", CONTRACT_NAME)
        .add_attribute("version", CONTRACT_VERSION)
    )
  }

  /// Contract execute entrypoint.
  #[cfg_attr(not(feature = "library"), entry_point)]
  pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecMsg) -> Result<Response> {
    msg.execute(deps.storage, deps.api, &env, &info)
  }

  /// Contract query entrypoint.
  #[cfg_attr(not(feature = "library"), entry_point)]
  pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    msg.query(deps.storage)
  }
}