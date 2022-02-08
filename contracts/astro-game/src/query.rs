use cosmwasm_std::{to_binary, Addr, Binary, Deps, StdResult, Order::Ascending as Ascending, Uint128};
use cw_storage_plus::{Bound, U64Key};

use crate::state::{AstroGameContract};
use crate::msg::QueryMsg;
impl<'a> AstroGameContract<'a> {
    pub fn query(&self, deps: Deps, msg: QueryMsg) -> StdResult<Binary> {
      match msg {
        QueryMsg::Config {} => to_binary(&self.config.load(deps.storage)?),
        QueryMsg::PoolInfo {pool_id} => to_binary(&self.pool_info.load(deps.storage, U64Key::new(pool_id))?)
      }
    }
  }