
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{from_binary, Coin, coin, Decimal, Uint128, };
use cw_storage_plus::{U64Key};

use crate::{
  state::{AstroGameContract},
  msg::{InstantiateMsg, QueryMsg, ExecuteMsg},
  error:: ContractError,
  asset::{Asset, AssetInfo}
};
#[test]
fn create_pool_test() {
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

  let support_asset:AssetInfo = AssetInfo::NativeToken { denom: "uusd".to_string()};
  let price_per_ticket_to_register:Asset = Asset {
    info: support_asset.clone(),
    amount: Uint128::from(100000000u128)
  };
  let create_pool_msg = ExecuteMsg::CreatePool {support_asset: support_asset.clone(), price_per_ticket_to_register: price_per_ticket_to_register.clone()};

  let _res = astro_game.execute(deps.as_mut(), mock_env(), info.clone(), create_pool_msg).unwrap();

  let pool_index = astro_game.pool_index.load(&deps.storage).unwrap();


  let pool_info = astro_game.pool_info.load(&deps.storage, U64Key::new(1)).unwrap();

  // check pool_index change
  assert_eq!(2, pool_index);
  assert_eq!(false, pool_info.claimed);
  assert_eq!(false, pool_info.is_closed);
  assert_eq!(None, pool_info.amount_reward);
  assert_eq!(support_asset.clone(), pool_info.support_asset);
  assert_eq!(price_per_ticket_to_register.clone(), pool_info.price_per_ticket_to_register);
  assert_eq!(0, pool_info.count_players);
  assert_eq!(None, pool_info.winner);

  
}