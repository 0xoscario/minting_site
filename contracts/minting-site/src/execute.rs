use std::env;
use std::ptr::NonNull;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{Config, MintingContract, Stage, TokenIds};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, BankMsg, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response,
    StdError, StdResult, Timestamp, WasmMsg,
};
use cw721::{Cw721ExecuteMsg};
use cw_storage_plus::Index;

impl<'a> MintingContract<'a> {
    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        let owner = msg
            .owner
            .map_or(Ok(_info.sender), |o| deps.api.addr_validate(&o))?;

        let config = Config {
            owner: Some(owner),
            treasury: deps.api.addr_validate(&msg.treasury)?,
            nft_contract_address: deps.api.addr_validate(&msg.nft_contract_address)?,
            collection_name: msg.collection_name,
            collection_symbol: msg.collection_symbol,
            minting_stages: msg.minting_stages,
        };

        let token_ids = TokenIds { token_ids: vec![] };
        self.config.save(deps.storage, &config)?;
        self.token_ids_list.save(deps.storage, &token_ids)?;

        Ok(Response::default())
    }

    pub fn execute(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::UpdateConfig { new_owner } => {
                self.execute_update_config(deps, _env, info, new_owner)
            }
            ExecuteMsg::AddToWhiteList {
                add_to_whitelists,
                stage_id,
            } => self.execute_add_to_whitelist(deps, _env, info, add_to_whitelists, stage_id),
            ExecuteMsg::AddTokenIds { token_ids } => {
                self.execute_add_token_ids_list(deps, _env, info, token_ids)
            }
            ExecuteMsg::RandomMint {} => self.execute_random_mint(deps, _env, info),
        }
    }
}

impl<'a> MintingContract<'a> {
    pub fn execute_update_config(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        new_owner: Option<String>,
    ) -> Result<Response, ContractError> {
        // authorize owner
        let cfg = self.config.load(deps.storage)?;
        let owner = cfg.owner.ok_or(ContractError::Unauthorized {})?;
        if info.sender != owner {
            return Err(ContractError::Unauthorized {});
        }

        // if owner some validated to addr, otherwise set to none
        let mut tmp_owner = None;
        if let Some(addr) = new_owner {
            tmp_owner = Some(deps.api.addr_validate(&addr)?)
        }

        self.config
            .update(deps.storage, |mut exists| -> StdResult<_> {
                exists.owner = tmp_owner;
                Ok(exists)
            })?;

        Ok(Response::new().add_attribute("action", "update_config"))
    }

    pub fn execute_random_mint(
        &self,
        deps: DepsMut,
        env: Env,
        _info: MessageInfo,
    ) -> Result<Response, ContractError> {
        let config = self.config.load(deps.storage)?;

        let current_stages: Vec<Stage> = config
            .minting_stages
            .into_iter()
            .filter(|t| {
                t.start_time <= env.block.time.seconds()
                    && t.end_time >= env.block.time.seconds()
            })
            .collect();

        let mut messages: Vec<CosmosMsg> = vec![];

        if current_stages.len() != 0 {
            let current_stage = current_stages[0].clone();

            let minting_price = current_stage.minting_price;

            // Check if some funds are sent
            let sent = match _info.funds.len() {
                0 => Err(StdError::generic_err(format!(
                    "you need to send {}{} per combination in order to register",
                    &minting_price.amount, &minting_price.denom
                ))),
                1 => {
                    if _info.funds[0].denom == minting_price.denom {
                        Ok(_info.funds[0].amount)
                    } else {
                        Err(StdError::generic_err(format!(
                            "you need to send {}{} per combination in order to register",
                            &minting_price.amount, &minting_price.denom
                        )))
                    }
                }
                _ => Err(StdError::generic_err(format!(
                    "Only send {0} to register",
                    &minting_price.denom
                ))),
            }?;

            if sent.is_zero() {
                return Err(ContractError::ZeroAmount {});
            }
            // Handle the player is not sending too much or too less
            if sent.u128() != u128::from(minting_price.amount) {
                return Err(ContractError::WrongAmount {});
            }

            //get list token Id
            let token_ids_list = self.token_ids_list.load(deps.storage)?;

            let length = token_ids_list.token_ids.clone().len();

            //get random id from block.timestamp
            let random_num: u64 = env.block.time.seconds() % length as u64;

            let token_id = &token_ids_list.token_ids[random_num as usize];

            //transfer NFT to sender
            messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: config.nft_contract_address.to_string(),
                msg: to_binary(&Cw721ExecuteMsg::TransferNft {
                  recipient: _info.sender.to_string(),
                  token_id: token_id.clone(),
                })?,
                funds: vec![]
              }));


        } else {
            return Err(ContractError::MintingEnded {});
        }


        Ok(
            Response::new().add_messages(messages)
                .add_attribute("action", "random_mint")
                .add_attribute("sender", _info.sender) 
        )
                             
    }

    pub fn execute_add_to_whitelist(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        add_to_whitelists: Vec<String>,
        stage_id: u8,
    ) -> Result<Response, ContractError> {
        // authorize owner
        let cfg = self.config.load(deps.storage)?;
        let owner = cfg.owner.ok_or(ContractError::Unauthorized {})?;
        if info.sender != owner {
            return Err(ContractError::Unauthorized {});
        }

        for white_list in add_to_whitelists {
            self.whitelist.save(
                deps.storage,
                (&deps.api.addr_validate(&white_list)?, &[stage_id]),
                &true,
            )?;
        }

        Ok(Response::new().add_attribute("action", "add_whitelist"))
    }

    fn execute_add_token_ids_list(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        token_ids: Vec<String>,
    ) -> Result<Response, ContractError> {
        // authorize owner
        let cfg = self.config.load(deps.storage)?;
        let owner = cfg.owner.ok_or(ContractError::Unauthorized {})?;
        if info.sender != owner {
            return Err(ContractError::Unauthorized {});
        }

        self.token_ids_list
            .update(deps.storage, |mut tok| -> StdResult<TokenIds> {
                tok.token_ids.extend(token_ids);
                Ok(tok)
            })?;
        Ok(Response::new().add_attribute("action", "add_token_ids"))
    }
}

