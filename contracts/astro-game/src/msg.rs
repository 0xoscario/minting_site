use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin, Decimal };
use crate::asset::{Asset,AssetInfo};
use cw20::Cw20ReceiveMsg;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct InstantiateMsg {
  pub owner: String,
  pub name: String,
  pub global_fee_rate: Decimal
}


#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {

  ReceiveToken(Cw20ReceiveMsg),

  UpdateConfig {
    owner: Option<String>,
    name: Option<String>,
    global_fee_rate: Option<Decimal>
  },
  CreatePool {
    price_per_ticket_to_register: Asset,
    support_asset: AssetInfo,
  },
  Register {
    pool_id: u64,
    price_per_ticket_to_register: Asset
  },
  SetWinner {
    pool_id: u64,
    winner: String
  }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Cw20HookMsg {
  Register {
    pool_id: u64,
  },

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
  Config {},
  PoolInfo {pool_id : u64},
}