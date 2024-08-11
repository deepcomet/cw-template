use {{crate_name}}::{cfg::CfgMsg, exec::ExecMsg, query::QueryMsg};
use cosmwasm_schema::write_api;

fn main() {
  write_api! {
    instantiate: CfgMsg,
    execute: ExecMsg,
    query: QueryMsg,
  }
}
