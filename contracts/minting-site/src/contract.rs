use crate::error::ContractError;
use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, WasmMsg,BankMsg
};
use cw2::set_contract_version;
use cw721_base::{
    msg::ExecuteMsg as Cw721ExecuteMsg, Extension,
    MintMsg,
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw721-fixed-price";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response>  {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = msg
        .owner
        .map_or(Ok(info.sender), |o| deps.api.addr_validate(&o))?;

    let config = Config {
        owner: Some(owner),
        treasury: deps.api.addr_validate(&msg.treasury)?,
        nft_token_address: deps.api.addr_validate(&msg.nft_token_address)?,
        collection_name: msg.collection_name,
        collection_symbol: msg.collection_symbol,
        price : msg.price
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default())
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_binary(&query_config(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        owner: config.owner,
        treasury: config.treasury,
        nft_token_address: config.nft_token_address,
        collection_name: config.collection_name,
        collection_symbol: config.collection_symbol,
        price : config.price
    })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateConfig { new_owner } => execute_update_config(deps, _env, info, new_owner),
        ExecuteMsg::MintNft { token_id, extension, token_uri, owner  } => {
            execute_mint_nft(deps, info, token_id, extension, token_uri, owner)
        }
    }
}

pub fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_owner: Option<String>,
) -> Result<Response, ContractError> {
    // authorize owner
    let cfg = CONFIG.load(deps.storage)?;
    let owner = cfg.owner.ok_or(ContractError::Unauthorized {})?;
    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }

    // if owner some validated to addr, otherwise set to none
    let mut tmp_owner = None;
    if let Some(addr) = new_owner {
        tmp_owner = Some(deps.api.addr_validate(&addr)?)
    }

    CONFIG.update(deps.storage, |mut exists| -> StdResult<_> {
        exists.owner = tmp_owner;
        Ok(exists)
    })?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

pub fn execute_mint_nft(
    deps: DepsMut,
    _info: MessageInfo,
    token_id: String,
    extension: Extension,
    token_uri: String,
    owner: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let transfer_msg = BankMsg::Send {
        to_address: config.treasury.to_string(),
        amount: vec![config.price.clone()],
    };

    let mint_msg = Cw721ExecuteMsg::Mint(MintMsg::<Extension> {
        token_id: token_id,
        owner: owner,
        token_uri: Some(token_uri),
        extension: extension,
    });

    let callback = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.nft_token_address.to_string(),
        msg: to_binary(&mint_msg)?,
        funds: vec![],
    });


    Ok(Response::new()
        .add_message(callback)
        .add_message(transfer_msg)
    )
}

