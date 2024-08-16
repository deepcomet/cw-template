use {{crate_name}}::{ExecMsg, InstantiateMsg, QueryMsg};
use cosmwasm_schema::write_api;

fn main() {
  write_api! {
    instantiate: InstantiateMsg,
    execute: ExecMsg,
    query: QueryMsg,
  }
}
