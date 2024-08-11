#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::cfg::CfgMsg;
use crate::error::Result;
use crate::exec::{set_cfg, ExecMsg};
use crate::query::{cfg, QueryMsg};
use crate::state::CFG;

/// Contract name (cargo package name).
const CONTRACT_NAME: &str = "crates.io:{{project-name}}";
/// Contract version (cargo package version).
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Contract instantiate entrypoint.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
  mut deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  msg: CfgMsg,
) -> Result<Response> {
  set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
  set_cfg(
    &mut deps,
    &msg,
    Some(
      Response::default()
        .add_attribute("method", "instantiate")
        .add_attribute("sender", info.sender.to_string())
        .add_attribute("name", CONTRACT_NAME)
        .add_attribute("version", CONTRACT_VERSION),
    ),
  )
}

/// Contract execute entrypoint.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(mut deps: DepsMut, _env: Env, info: MessageInfo, msg: ExecMsg) -> Result<Response> {
  let res = Response::default()
    .add_attribute("method", "execute")
    .add_attribute("sender", info.sender.to_string());
  match msg {
    ExecMsg::SetCfg(m) => {
      CFG.load(deps.storage)?.authorize_admin(&info.sender)?;
      set_cfg(&mut deps, &m, Some(res))
    }
  }
}

/// Contract query entrypoint.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
  match msg {
    QueryMsg::Cfg {} => to_json_binary(&cfg(&deps)?),
  }
}
