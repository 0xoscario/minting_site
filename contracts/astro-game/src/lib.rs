pub mod execute;
mod error;
pub mod msg;
pub mod state;
pub mod query;
pub mod asset;
// pub use crate::error::ContractError;

#[cfg(test)]
pub mod testing;

use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg};
use crate::state::AstroGameContract;
use crate::error::ContractError;

#[cfg(not(feature = "library"))]
pub mod entry {
  use super::*;

  use cosmwasm_std::entry_point;
  use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

  #[entry_point]
  pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
  ) -> StdResult<Response> {
    let tract = AstroGameContract::default();
    tract.instantiate(deps, env, info, msg)
  }

  #[entry_point]
  pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
  ) -> Result<Response, ContractError> {
    let tract = AstroGameContract::default();
    tract.execute(deps, env, info, msg)
  }

  #[entry_point]
  pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let tract = AstroGameContract::default();
   
    tract.query(deps, msg)
  }

}