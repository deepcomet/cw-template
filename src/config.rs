use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Api};

use crate::error::{IntoResult, Result};
use crate::util::authorize_addr;

/// Contract config settings.
#[cw_serde]
pub struct Config {
  /// Contract admins are authorized to update config and perform privileged actions.
  pub admins: Vec<Addr>,
}

impl Config {
  /// Checks if the given address is in the contract admins list.
  ///
  /// # Examples
  ///
  /// ```
  /// let addr = cosmwasm_std::Addr::unchecked("example123abc");
  /// let mut config = {{crate_name}}::Config::default();
  /// let auth_err = config.authorize_admin(&addr);
  /// assert!(auth_err.is_err());
  /// config.admins.push(addr.clone());
  /// let auth_ok = config.authorize_admin(&addr);
  /// assert!(auth_ok.is_ok());
  /// ```
  pub fn authorize_admin(&self, addr: &Addr) -> Result {
    authorize_addr(addr, &self.admins)
  }
}

impl Default for Config {
  /// Config with default settings.
  fn default() -> Self {
    Self { admins: vec![] }
  }
}

impl Into<ConfigMsg> for Config {
  /// Convert `Config` to message interface `ConfigMsg`.
  ///
  /// # Examples
  ///
  /// ```
  /// let mut config = {{crate_name}}::Config::default();
  /// config.admins.push(cosmwasm_std::Addr::unchecked("example123abc"));
  /// let config_msg: {{crate_name}}::ConfigMsg = config.into();
  /// assert_eq!(config_msg.admins, vec!["example123abc"]);
  /// ```
  fn into(self) -> ConfigMsg {
    ConfigMsg {
      admins: self.admins.iter().map(String::from).collect(),
    }
  }
}

/// Message interface for contract config.
#[cw_serde]
pub struct ConfigMsg {
  /// Contract admins in bech32 string format.
  pub admins: Vec<String>,
}

impl ConfigMsg {
  /// Convert message interface `ConfigMsg` to validated `Config`.
  ///
  /// # Examples
  ///
  /// ```
  /// let deps = cosmwasm_std::testing::mock_dependencies();
  /// let mut msg = {{crate_name}}::ConfigMsg::default();
  /// let addr = deps.api.addr_make("test");
  /// msg.admins.push(addr.to_string());
  /// let config_ok = msg.validate(&deps.api);
  /// assert_eq!(config_ok.unwrap().admins, vec![addr]);
  /// msg.admins.push("example123abc".to_string());
  /// let config_err = msg.validate(&deps.api);
  /// assert!(config_err.is_err());
  /// ```
  pub fn validate(&self, api: &dyn Api) -> Result<Config> {
    Ok(Config {
      admins: self
        .admins
        .iter()
        .map(|admin| api.addr_validate(&admin).into_result())
        .collect::<Result<Vec<Addr>>>()?,
    })
  }
}

impl Default for ConfigMsg {
  /// Message interface with default config settings.
  fn default() -> Self {
    Config::default().into()
  }
}
