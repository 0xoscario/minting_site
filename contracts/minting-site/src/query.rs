use cosmwasm_std::{to_binary, Addr, Binary, Deps, StdResult, Order::Ascending as Ascending, };

use crate::msg::QueryMsg;
use crate::state::MintingContract;

impl<'a> MintingContract<'a> {
    pub fn query(&self, deps: Deps, msg: QueryMsg) -> StdResult<Binary> {
      match msg {
        QueryMsg::Config {} => to_binary(&self.config.load(deps.storage)?),
        QueryMsg::Whitelist{address, stage_id} => to_binary(&self.whitelist.load(deps.storage, (&deps.api.addr_validate(&address)?, &[stage_id]))?),
        QueryMsg::TokenIdsList {} => to_binary(&self.token_ids_list.load(deps.storage)?),

      }
    }
  }




