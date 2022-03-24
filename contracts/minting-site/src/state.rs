use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin, StdResult, Env, Deps};
use cw_storage_plus::{Item, Map,};

pub struct MintingContract<'a> {
       pub config: Item<'a, Config>,
       pub whitelist: Map<'a, (&'a Addr, &'a[u8]), bool>,
       pub token_ids_list: Item<'a, TokenIds>,
 }
     
impl Default for MintingContract<'static> {
    fn default() -> Self {
         Self::new(
           "config",
           "whitelist",
           "token_ids_key"
         )
       }
}

impl<'a> MintingContract<'a> {
       fn new(
         config_key: &'a str,
         whitelist_key: &'a str,
         token_ids_key: &'a str,
       ) -> Self {
         Self {
           config: Item::new(config_key),
           whitelist: Map::new(whitelist_key),
           token_ids_list: Item::new(token_ids_key),
         }
       }
     }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
       /// Owner If None set, contract is frozen.
       pub owner: Option<Addr>,
       // Address who receipt luna for minting 
       pub treasury: Addr,
       //Address of contract nft
       pub nft_contract_address: Addr,
       //Collection Name
       pub collection_name: String,
       //collection symbol 
       pub collection_symbol: String,
       //Price of nft for minting
       pub minting_stages: Vec<Stage>, 

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenIds {
       pub token_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Stage {
       pub stage_name: String,
       pub stage_id: u8,
       pub start_time: u64,
       pub end_time: u64,
       pub check_whitelisted: bool,
       pub minting_price: Coin,
}
