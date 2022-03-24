use std::string;

use cw721_base::Extension;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map, U8Key};
use crate::state::{Stage};
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: Option<String>,
    // Address who receipt luna for minting 
    pub treasury: String,
    //Address of contract nft
    pub nft_contract_address: String,
    //Collection Name
    pub collection_name: String,
    //collection symbol 
    pub collection_symbol: String,
    //Price of nft for minting
    pub minting_stages: Vec<Stage>,

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateConfig {
        /// NewOwner if non sent, contract gets locked. Recipients can receive airdrops
        /// but owner cannot register new stages.
        new_owner: Option<String>,
    },
    AddToWhiteList {
        add_to_whitelists: Vec<String>,
        stage_id: u8,
    },
    AddTokenIds{
        token_ids: Vec<String>,
    },

    RandomMint {},

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    Whitelist {address: String, stage_id: u8},
    TokenIdsList{}
}