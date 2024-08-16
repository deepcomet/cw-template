use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Api, Env, MessageInfo, Response, Storage};

use crate::config::ConfigMsg;
use crate::error::Result;
use crate::state::update_config;

/// Contract execute message.
#[cw_serde]
pub enum ExecMsg {
  /// Sets contract config.
  ///
  /// Requires admin authorization.
  ///
  /// See implementation: [`configure`].
  Configure(ConfigMsg),
}

impl ExecMsg {
  /// Execute the message.
  pub fn execute(
    &self,
    storage: &mut dyn Storage,
    api: &dyn Api,
    _env: &Env,
    info: &MessageInfo,
  ) -> Result<Response> {
    let res = Response::default()
      .add_attribute("method", "execute")
      .add_attribute("sender", info.sender.to_string());
    match self {
      ExecMsg::Configure(msg) => configure(storage, api, info, &msg, res),
    }
  }
}

/// Implementation of [`ExecMsg::Configure`].
fn configure(
  storage: &mut dyn Storage,
  api: &dyn Api,
  info: &MessageInfo,
  msg: &ConfigMsg,
  res: Response,
) -> Result<Response> {
  let config = msg.validate(api)?;
  update_config(storage, |prev_config| {
    prev_config.authorize_admin(&info.sender)?;
    Ok(config)
  })?;
  Ok(res.add_attribute("action", "configure"))
}
