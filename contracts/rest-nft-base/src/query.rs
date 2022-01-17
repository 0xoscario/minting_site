use cosmwasm_std::{Deps, StdResult};


use crate::state::{Config, CONFIG};
//use rest_nft::msg::{ReserveResponse};

pub fn query_config(deps: Deps) -> StdResult<Config> {
    CONFIG.load(deps.storage)
}

pub fn query_frozen(deps: Deps) -> StdResult<bool> {
    let config = CONFIG.load(deps.storage)?;
    Ok(config.frozen)
}

pub fn query_reserved(deps: Deps) -> StdResult<i32> {
    let config = CONFIG.load(deps.storage)?;
    Ok(config.reserved_tokens)
}
