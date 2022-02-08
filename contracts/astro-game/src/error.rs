use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Pool is closed")]
    PoolClosed {},

    #[error("Global fee rate is higher than 100%")]
    InvalidFeeRate {},

    #[error("Asset type mismatch")]
    AssetInfoMismatch {},

    #[error("You must send exactly amount to the pool")]
    AssetAmountMismatch {},

    #[error("Only winner can claim reward")]
    NotWinner {},
}
