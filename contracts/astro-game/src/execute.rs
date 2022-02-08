use cosmwasm_std::{from_binary, to_binary, Addr, CosmosMsg, Decimal, Deps, DepsMut, Env, MessageInfo, StdError, StdResult, Response, WasmMsg, Uint128, Coin};
use cw_storage_plus::{U64Key};
use crate::msg::{InstantiateMsg, ExecuteMsg, Cw20HookMsg};
use crate::error::ContractError;
use crate::state::{Config, AstroGameContract, PoolInfo};
use crate::asset::{Asset, AssetInfo};
use cw20::Cw20ReceiveMsg;
impl<'a> AstroGameContract<'a> {
    pub fn instantiate(
      &self,
      deps: DepsMut,
      _env: Env,
      _info: MessageInfo,
      msg: InstantiateMsg
    ) -> StdResult<Response> {

      if msg.global_fee_rate > Decimal::one() {
        return Err(StdError::generic_err("Cancel fee rate can't exceed 1"))
      }
      let config = Config {
        owner: deps.api.addr_validate(msg.owner.as_str())?,
        name: msg.name,
        global_fee_rate: msg.global_fee_rate
      };
  
      self.config.save(deps.storage, &config)?;
  
      let pool_index = 1u64;
      self.pool_index.save(deps.storage, &pool_index)?;
      Ok(Response::new())
    }

    pub fn execute(
      &self,
      deps: DepsMut,
      _env: Env, 
      info: MessageInfo,
      msg: ExecuteMsg
    ) -> Result<Response, ContractError> {
      match msg {
        ExecuteMsg::CreatePool{price_per_ticket_to_register, support_asset} => self.create_pool(deps, _env, info, price_per_ticket_to_register, support_asset),
        ExecuteMsg::ReceiveToken(msg) => self.receive_token(deps, _env, info, msg),
        ExecuteMsg::UpdateConfig {owner, name, global_fee_rate} => self.update_config(deps, _env, info, owner, name, global_fee_rate),
        ExecuteMsg::Register {pool_id, price_per_ticket_to_register} => self.register_pool(deps, _env, info.clone(), info.sender, pool_id, price_per_ticket_to_register),
        ExecuteMsg::SetWinner {pool_id, winner} => self.set_winner(deps, _env, info, pool_id, winner),
      }
    }
}

impl<'a> AstroGameContract <'a> {

  pub fn receive_token(
    &self, 
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: Cw20ReceiveMsg
  ) -> Result<Response, ContractError> {
    let contract_addr = info.sender.clone();

    let sender = deps.api.addr_validate(&msg.sender)?;

    let cw20_msg = from_binary::<Cw20HookMsg>(&msg.msg)?;

    let price_per_ticket_to_register = Asset {
      info: AssetInfo::Token { contract_addr: contract_addr.to_string() },
      amount: msg.amount
    };

    match cw20_msg {
      Cw20HookMsg::Register { pool_id } 
        => self.register_pool(deps, _env, info, sender, pool_id, price_per_ticket_to_register),
   
    }
  }

  pub fn create_pool(
    &self, 
    deps: DepsMut, 
    _env: Env, 
    info: MessageInfo,
    price_per_ticket_to_register: Asset,
    support_asset: AssetInfo,
  ) -> Result<Response, ContractError> {
    let config:Config = self.config.load(deps.storage)?;
    if info.sender != config.owner {
      return Err(ContractError::Unauthorized {})
    }

    let pool_index = self.pool_index.load(deps.storage)?;

    let pool_info =  PoolInfo {
      claimed: false,
      is_closed: false,
      amount_reward : None,
      support_asset: support_asset,
      price_per_ticket_to_register: price_per_ticket_to_register.clone(),
      count_players: 0,
      players : vec![],
      winner: None,
    };

    let key = U64Key::new(pool_index);

    self.pool_info.save(deps.storage, key, &pool_info)?;
    self.pool_index.save(deps.storage, &(pool_index + 1))?;
    
    Ok(Response::new()
      .add_attribute("action", "create_new_pool")
      .add_attribute("pool_id", pool_index.to_string())
      .add_attribute("price_per_ticket_to_register", format!("{}", price_per_ticket_to_register.clone()))
    )
  }

  pub fn update_config(
    &self, 
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    owner: Option<String>,
    name: Option<String>,
    global_fee_rate: Option<Decimal>
  ) -> Result<Response, ContractError> {
    let mut config:Config = self.config.load(deps.storage)?;
    if info.sender != config.owner {
      return Err(ContractError::Unauthorized {})
    }

    if let Some(owner) = owner {
      config.owner = deps.api.addr_validate(&owner)?;
    }

    if let Some(name) = name {
      config.name = name;
    }

    if let Some(global_fee_rate) = global_fee_rate {
      if global_fee_rate > Decimal::one() {
        return Err(ContractError::InvalidFeeRate {})
      }

      config.global_fee_rate = global_fee_rate;
    }

    self.config.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))

  }

  pub fn register_pool(
    &self, 
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    sender: Addr,
    pool_id: u64,
    price_per_ticket_to_register: Asset
  ) -> Result<Response, ContractError> {
    let key = U64Key::new(pool_id);
    let mut pool_info = self.pool_info.load(deps.storage, key)?;
    if pool_info.is_closed {
      return Err(ContractError::PoolClosed{})
    }

    price_per_ticket_to_register.assert_sent_native_token_balance(&info)?;

    if price_per_ticket_to_register.info != pool_info.price_per_ticket_to_register.info {
      return Err(ContractError::AssetInfoMismatch {})

    }

    if price_per_ticket_to_register.amount != pool_info.price_per_ticket_to_register.amount {
      return Err(ContractError::AssetAmountMismatch {})
    }
    
    // 
    pool_info.count_players = &pool_info.count_players + 1; 

    pool_info.players.push(sender.clone());

    self.pool_info.save(deps.storage, U64Key::new(pool_id), &pool_info)?;
    
    Ok(Response::new()
    .add_attribute("action", "register_pool")
    .add_attribute("sender", sender.to_string())
    .add_attribute("pool_id", pool_id.to_string())
    )
  }


  pub fn set_winner(
    &self, 
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    pool_id: u64,
    winner: String
  ) -> Result<Response, ContractError> {
    let config = self.config.load(deps.storage)?;
    if info.sender != config.owner {
      return Err(ContractError::Unauthorized {})
    }

    let key = U64Key::new(pool_id);

    let mut pool_info = self.pool_info.load(deps.storage, key)?;

    pool_info.is_closed = true;

    pool_info.winner = Some(deps.api.addr_validate(&winner.clone())?);

    let amount_reward = Uint128::from(pool_info.count_players) * pool_info.price_per_ticket_to_register.amount; 

    pool_info.amount_reward = Some(Asset {
      info: pool_info.support_asset.clone(),
      amount: amount_reward
    });

    self.pool_info.save(deps.storage, U64Key::new(pool_id), &pool_info)?;

    Ok(Response::new()
    .add_attribute("action", "set_winner")
    .add_attribute("winner", winner.clone())
    .add_attribute("pool_id", pool_id.to_string())
    .add_attribute("amount_reward", format!("{}", amount_reward.clone()))
    )

  }

  pub fn claim_reward(
    &self, 
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    pool_id: u64,
  ) -> Result<Vec<CosmosMsg>, ContractError> {
    let key = U64Key::new(pool_id);

    let config = self.config.load(deps.storage)?;
    let mut pool_info = self.pool_info.load(deps.storage, key)?;

    let mut messages: Vec<CosmosMsg> = vec![];

    let amount_reward = pool_info.amount_reward.clone();


    let winner = pool_info.winner.clone();
     
    if let Some (winner) = winner {
      if info.sender != winner {
        return Err(ContractError::NotWinner {})

      }

      if let Some(mut amount_reward) = amount_reward {
        let fee_amount = amount_reward.amount * config.global_fee_rate; 
        
        amount_reward.amount = amount_reward.amount - fee_amount;

        let amount_claim = Asset {
          info: amount_reward.info.clone(),
          amount: amount_reward.amount,
        }; 
       messages.push(amount_claim.into_msg(&deps.querier, winner)?);

       pool_info.claimed = true;

       self.pool_info.save(deps.storage, U64Key::new(pool_id), &pool_info)?;

      }
  
    }

    Ok(messages)

  }
}