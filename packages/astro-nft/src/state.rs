use cosmwasm_std::Empty;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub type AstroNFTContract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty>;
pub type Extension = Option<Metadata>;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Trait {
    pub trait_type: String,
    pub value: String,
}

// see: https://docs.opensea.io/docs/metadata-standards
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Metadata {
    pub name: Option<String>,
    pub faction: Option<String>,
    pub attributes: Option<Vec<Trait>>,

}
