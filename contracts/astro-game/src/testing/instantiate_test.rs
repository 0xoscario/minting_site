use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{from_binary, Coin, coin, Decimal};

use crate::{
  state::{AstroGameContract, Config},
  msg::{InstantiateMsg, QueryMsg},
};

#[test]
fn instantiate_test() {
  let astro_game = AstroGameContract::default();

  let mut deps = mock_dependencies(&[]);

  let instantiate_msg = InstantiateMsg {
    owner: "owner".to_string(),
    name: "astro_game".to_string(),
    global_fee_rate: Decimal::from_ratio(10u128, 100u128),
  };

  let info = mock_info("owner", &[]);

  let _res = astro_game.instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();

  let config: Config = from_binary(
    &astro_game.query(deps.as_ref(), QueryMsg::Config {}).unwrap()
  ).unwrap();

  assert_eq!("owner".to_string(), config.owner);
  assert_eq!("astro_game".to_string(), config.name);
  assert_eq!(Decimal::from_ratio(10u128, 100u128), config.global_fee_rate);

  let pool_index = astro_game.pool_index.load(&deps.storage).unwrap();

  assert_eq!(1, pool_index);
}