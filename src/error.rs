use cosmwasm_std::{CoinsError, StdError, StdResult};
use miette::Diagnostic;

/// Type alias for `std::result::Result` with contract defaults.
pub type Result<T=(), E=Error> = std::result::Result<T, E>;

/// Contract error.
#[derive(thiserror::Error, Debug, Diagnostic)]
pub enum Error {
  /// CosmWasm standard error.
  #[error(transparent)]
  Std(#[from] StdError),

  /// CosmWasm coin parsing or manipulation error.
  #[error(transparent)]
  Coins(#[from] CoinsError),

  /// Unauthorized user error.
  #[error("User is not authorized for this action")]
  Unauthorized {},

  /// Action disabled error.
  #[error("This action is disabled")]
  Disabled {},

  /// Input parsing error.
  #[error("Unable to parse input value")]
  Parse {},
}

impl Into<StdError> for Error {
  /// Convert contract error into CosmWasm standard error.
  fn into(self) -> StdError {
    match self {
      Error::Std(err) => err,
      _ => StdError::generic_err(self.to_string()),
    }
  }
}

/// Trait for conversions between result types.
pub trait IntoResult<T, E> {
  /// Convert result to target type.
  fn into_res(self) -> Result<T, E>;
}

impl<T> IntoResult<T, StdError> for Result<T, Error> {
  /// Convert contract result to CosmWasm standard result.
  fn into_res(self) -> StdResult<T> {
    self.map_err(Into::into)
  }
}

impl<T> IntoResult<T, Error> for StdResult<T> {
  /// Convert CosmWasm standard result to contract result.
  fn into_res(self) -> Result<T, Error> {
    self.map_err(Error::Std)
  }
}
