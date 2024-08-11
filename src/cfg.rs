use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Deps};

use crate::error::{IntoResult, Result};
use crate::util::authorize_addr;

/// Contract config settings.
#[cw_serde]
pub struct Cfg {
  /// Contract admins are authorized to update config and perform privileged actions.
  pub admins: Vec<Addr>,
}

impl Cfg {
  /// Checks if the given address is in the contract admins list.
  /// Returns `Ok(())`` if authorized, or `Err(Error::Unauthorized)` if not.
  ///
  /// # Examples
  ///
  /// ```
  /// let addr = cosmwasm_std::Addr::unchecked("example123abc");
  /// let mut cfg = {{crate_name}}::cfg::Cfg::default();
  /// let auth_err = cfg.authorize_admin(&addr);
  /// assert!(auth_err.is_err());
  /// cfg.admins.push(addr.clone());
  /// let auth_ok = cfg.authorize_admin(&addr);
  /// assert!(auth_ok.is_ok());
  /// ```
  pub fn authorize_admin(&self, addr: &Addr) -> Result {
    authorize_addr(addr, &self.admins)
  }
}

impl Default for Cfg {
  /// Config with default settings.
  fn default() -> Self {
    Self { admins: vec![] }
  }
}

impl Into<CfgMsg> for Cfg {
  /// Convert `Cfg` to message interface `CfgMsg`.
  ///
  /// # Examples
  ///
  /// ```
  /// let mut cfg = {{crate_name}}::cfg::Cfg::default();
  /// cfg.admins.push(cosmwasm_std::Addr::unchecked("example123abc"));
  /// let cfg_msg: {{crate_name}}::cfg::CfgMsg = cfg.into();
  /// assert_eq!(cfg_msg.admins, vec!["example123abc"]);
  /// ```
  fn into(self) -> CfgMsg {
    CfgMsg {
      admins: self.admins.iter().map(String::from).collect(),
    }
  }
}

/// Message interface for contract config.
#[cw_serde]
pub struct CfgMsg {
  /// Contract admins in bech32 string format.
  pub admins: Vec<String>,
}

impl CfgMsg {
  /// Convert message interface `CfgMsg` to validated `Cfg`.
  ///
  /// # Examples
  ///
  /// ```
  /// let storage = cosmwasm_std::MemoryStorage::new();
  /// let api = cosmwasm_std::testing::MockApi::default().with_prefix("example");
  /// let querier = cosmwasm_std::testing::MockQuerier::default();
  /// let deps = cosmwasm_std::OwnedDeps{ storage, api, querier, custom_query_type: core::marker::PhantomData };
  /// let mut msg = {{crate_name}}::cfg::CfgMsg::default();
  /// let addr = api.addr_make("test");
  /// msg.admins.push(addr.to_string());
  /// let cfg_ok = msg.validate(&deps.as_ref());
  /// assert_eq!(cfg_ok.unwrap().admins, vec![addr]);
  /// msg.admins.push("example123abc".to_string());
  /// let cfg_err = msg.validate(&deps.as_ref());
  /// assert!(cfg_err.is_err());
  /// ```
  pub fn validate(&self, deps: &Deps) -> Result<Cfg> {
    Ok(Cfg {
      admins: self
        .admins
        .iter()
        .map(|admin| deps.api.addr_validate(&admin).into_res())
        .collect::<Result<Vec<Addr>>>()?,
    })
  }
}

impl Default for CfgMsg {
  /// Message interface with default config settings.
  fn default() -> Self {
    Cfg::default().into()
  }
}
