use cosmwasm_std::Addr;

use crate::error::{Error, Result};

/// Validates that the address is in the `authorized` list.
///
/// # Examples
///
/// ```
/// let admin_addr = cosmwasm_std::Addr::unchecked("example123abc");
/// let authorized = vec![admin_addr.clone()];
/// let auth_ok = {{crate_name}}::util::authorize_addr(&admin_addr, &authorized);
/// assert!(auth_ok.is_ok());
/// let other_addr = cosmwasm_std::Addr::unchecked("example456def");
/// let auth_err = {{crate_name}}::util::authorize_addr(&other_addr, &authorized);
/// assert!(auth_err.is_err());
/// ```
pub fn authorize_addr(addr: &Addr, authorized: &Vec<Addr>) -> Result {
  if !authorized.iter().any(|a| a == addr) {
    return Err(Error::Unauthorized {});
  }
  Ok(())
}
