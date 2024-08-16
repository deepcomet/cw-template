use cosmwasm_std::{
  to_json_binary, Addr, Coin, CosmosMsg, CustomQuery, Querier, QuerierWrapper, StdResult,
  WasmMsg, WasmQuery
};
use cosmwasm_schema::cw_serde;
use serde::de::DeserializeOwned;

use crate::{ConfigMsg, ExecMsg, QueryMsg};

/// API wrapper provides helper functions for interacting with the contract.
#[cw_serde]
pub struct {{project-name-pascal}} {
  /// Contract address.
  pub addr: Addr,
}

impl {{project-name-pascal}} {
  // ======= EXEC =======
  /// Create an execute message in the proper format from the given data. See [`ExecMsg`].
  /// 
  /// # Examples
  /// 
  /// ```
  /// let exec_msg = {{crate_name}}::ExecMsg::Configure({{crate_name}}::ConfigMsg::default());
  /// let contract: {{crate_name}}::{{project-name-pascal}} = cosmwasm_std::Addr::unchecked("example123abc").into();
  /// let cosmos_msg = contract.exec(exec_msg, vec![]).unwrap();
  /// let cosmwasm_std::CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute { contract_addr, funds, .. }) =
  ///   cosmos_msg else { panic!("Unable to parse message") };
  /// assert_eq!(contract_addr, contract.addr.to_string());
  /// assert_eq!(funds, vec![]);
  /// ```
  pub fn exec<T: Into<ExecMsg>>(&self, msg: T, funds: Vec<Coin>) -> StdResult<CosmosMsg> {
    Ok(
      WasmMsg::Execute {
        contract_addr: self.addr.to_string(),
        msg: to_json_binary(&msg.into())?,
        funds,
      }
      .into(),
    )
  }

  /// Create message for [`ExecMsg::Configure`].
  pub fn configure(&self, msg: ConfigMsg) -> StdResult<CosmosMsg> {
    self.exec(ExecMsg::Configure(msg), vec![])
  }

  // ======= QUERY =======
  /// Perform given query. See [`QueryMsg`].
  pub fn query<Q: Querier, C: CustomQuery, R: DeserializeOwned>(
    &self,
    querier: &Q,
    msg: &QueryMsg,
  ) -> StdResult<R> {
    QuerierWrapper::<C>::new(querier).query::<R>(
      &WasmQuery::Smart {
        contract_addr: self.addr.to_string(),
        msg: to_json_binary(msg)?,
      }
      .into(),
    )
  }

  /// See [`QueryMsg::Config`].
  pub fn config<Q: Querier, C: CustomQuery>(&self, querier: &Q) -> StdResult<ConfigMsg> {
    self.query::<Q, C, ConfigMsg>(querier, &QueryMsg::Config {})
  }
}

impl From<Addr> for {{project-name-pascal}} {
  /// Convert an address to a contract API wrapper.
  /// 
  /// # Examples
  /// 
  /// ```
  /// let addr = cosmwasm_std::Addr::unchecked("example123abc");
  /// let contract: {{crate_name}}::{{project-name-pascal}} = addr.clone().into();
  /// assert_eq!(contract.addr, addr);
  /// ```
  fn from(addr: Addr) -> Self {
    Self { addr }
  }
}
