use cw721_base::Extension;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map, U8Key};
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: Option<String>,
    // Address who receipt luna for minting 
    pub treasury: String,
    //Address of contract nft
    pub nft_token_address: String,
    //Collection Name
    pub collection_name: String,
    //collection symbol 
    pub collection_symbol: String,
    //Price of nft for minting
    pub price: Coin   

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateConfig {
        /// NewOwner if non sent, contract gets locked. Recipients can receive airdrops
        /// but owner cannot register new stages.
        new_owner: Option<String>,
    },
    /// Claim does not check if contract has enough funds, owner must ensure it.
    MintNft {
        token_id: String,
        extension: Extension,
        token_uri: String,
        owner: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetConfig {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
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
