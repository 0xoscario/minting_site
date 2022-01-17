use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map, U8Key};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
       /// Owner If None set, contract is frozen.
       pub owner: Option<Addr>,
       // Address who receipt luna for minting 
       pub treasury: Addr,
       //Address of contract nft
       pub nft_token_address: Addr,
       //Collection Name
       pub collection_name: String,
       //collection symbol 
       pub collection_symbol: String,
       //Price of nft for minting
       pub price: Coin   

}

pub const CONFIG: Item<Config> = Item::new("config");
