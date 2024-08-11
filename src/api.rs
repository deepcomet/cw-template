use cosmwasm_std::{to_json_binary, Addr, CosmosMsg, StdResult, WasmMsg, Coin};
use cosmwasm_schema::cw_serde;

use crate::exec::ExecMsg;

/// API wrapper provides helper functions for interacting with the contract.
#[cw_serde]
pub struct {{project-name-pascal}} {
  /// Contract address.
  pub addr: Addr,
}

impl {{project-name-pascal}} {
  /// Create an execute message in the proper format from the given data.
  /// 
  /// # Examples
  /// 
  /// ```
  /// let exec_msg = {{crate_name}}::exec::ExecMsg::SetCfg({{crate_name}}::cfg::CfgMsg::default());
  /// let api: {{crate_name}}::{{project-name-pascal}} = cosmwasm_std::Addr::unchecked("example123abc").into();
  /// let cosmos_msg = api.exec(exec_msg, vec![]).unwrap();
  /// let cosmwasm_std::CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute { contract_addr, funds, .. }) =
  ///   cosmos_msg else { panic!("Unable to parse message") };
  /// assert_eq!(contract_addr, api.addr.to_string());
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
