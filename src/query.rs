use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Deps, StdResult};

use crate::cfg::CfgMsg;
use crate::state::CFG;

/// Contract query message.
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
  /// Returns the active contract config.
  #[returns(CfgMsg)]
  Cfg {},
}

/// Query config implementation.
pub fn cfg(deps: &Deps) -> StdResult<CfgMsg> {
  Ok(CFG.load(deps.storage)?.into())
}
