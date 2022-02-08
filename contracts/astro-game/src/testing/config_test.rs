
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{from_binary, Coin, coin, Decimal};

use crate::{
  state::{AstroGameContract, Config},
  msg::{InstantiateMsg, QueryMsg, ExecuteMsg},
  error:: ContractError
};
#[test]
fn update_config_test() {
  // instantiate
  let astro_game = AstroGameContract::default();

  let mut deps = mock_dependencies(&[]);

  let instantiate_msg = InstantiateMsg {
    owner: "owner".to_string(),
    name: "astro_game".to_string(),
    global_fee_rate: Decimal::from_ratio(10u128, 100u128),
  };

  let info = mock_info("owner", &[]);

  let _res = astro_game.instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap();

  // update config, owner change
  let update_config_msg = ExecuteMsg::UpdateConfig {
    owner: Some("next_owner".to_string()),
    name: None,
    global_fee_rate: None,
  };

  let _res = astro_game.execute(deps.as_mut(), mock_env(), info.clone(), update_config_msg).unwrap();

  let config = astro_game.config.load(&deps.storage).unwrap();

  // check onwer changed
  assert_eq!("next_owner".to_string(), config.owner);

  // former owner try to update config
  
  let update_config_msg = ExecuteMsg::UpdateConfig {
    owner: Some("owner".to_string()),
    name: None,
    global_fee_rate: None,
  };

  let res = astro_game.execute(deps.as_mut(), mock_env(), info.clone(), update_config_msg);

  match res {
    Err(ContractError::Unauthorized {}) => assert!(true),
    _ => panic!("Must return unauthorized error"),
  }

  // update config. other options
  let update_config_msg = ExecuteMsg::UpdateConfig {
    owner: None,
    name: Some("new_name".to_string()),
    global_fee_rate: Some(Decimal::from_ratio(5u128, 100u128)),
  };

  let info = mock_info("next_owner", &[]);

  let _res = astro_game.execute(deps.as_mut(), mock_env(), info.clone(), update_config_msg).unwrap();

  let config = astro_game.config.load(&deps.storage).unwrap();

  assert_eq!("new_name".to_string(), config.name);
  assert_eq!(Decimal::from_ratio(5u128, 100u128), config.global_fee_rate);
}