use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{to_json_binary, Binary, StdResult, Storage};

use crate::state::get_config;
use crate::{ConfigMsg, IntoResult};

/// Contract query message.
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
  /// Query the contract config.
  ///
  /// See implementation: [`config`].
  #[returns(ConfigMsg)]
  Config {},
}

impl QueryMsg {
  /// Perform the query
  pub fn query(self, storage: &dyn Storage) -> StdResult<Binary> {
    match self {
      QueryMsg::Config {} => to_json_binary(&config(storage)?),
    }
  }
}

/// Implementation of [`QueryMsg::Config`].
fn config(storage: &dyn Storage) -> StdResult<ConfigMsg> {
  get_config(storage).map(Into::into).into_result()
}
