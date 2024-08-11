use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DepsMut, Response};

use crate::cfg::CfgMsg;
use crate::error::Result;
use crate::state::CFG;

/// Contract execute message.
#[cw_serde]
pub enum ExecMsg {
  /// Sets contract config to the given value. Requires admin authorization.
  SetCfg(CfgMsg),
}

/// Set config implementation.
pub fn set_cfg(
  deps: &mut DepsMut,
  msg: &CfgMsg,
  extend_res: Option<Response>,
) -> Result<Response> {
  let new_cfg = msg.validate(&deps.as_ref())?;
  CFG.save(deps.storage, &new_cfg)?;
  let res = extend_res.unwrap_or(Response::default());
  Ok(res.add_attribute("action", "set_cfg"))
}
