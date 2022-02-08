
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{from_binary, Coin, coin, Decimal, Uint128,Addr };
use cw_storage_plus::{U64Key};

use crate::{
  state::{AstroGameContract},
  msg::{InstantiateMsg, QueryMsg, ExecuteMsg},
  error:: ContractError,
  asset::{Asset, AssetInfo}
};
#[test]
fn register_pool_test() {
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
  //create pool
  let create_pool_msg = ExecuteMsg::CreatePool {support_asset: support_asset.clone(), price_per_ticket_to_register: price_per_ticket_to_register.clone()};

  let _res = astro_game.execute(deps.as_mut(), mock_env(), info.clone(), create_pool_msg).unwrap();

  //register pool 
  let info = mock_info("player1", &[Coin{ denom: "uusd".to_string(), amount: Uint128::from(100000000u128) }]);

  let register_pool_msg = ExecuteMsg::Register {
      pool_id : 1,
      price_per_ticket_to_register: price_per_ticket_to_register.clone()
  };

  let res = astro_game.execute(deps.as_mut(), mock_env(), info, register_pool_msg.clone());


  let info = mock_info("player2", &[Coin{ denom: "uusd".to_string(), amount: Uint128::from(100000000u128) }]);
  let res = astro_game.execute(deps.as_mut(), mock_env(), info, register_pool_msg.clone());

  let pool_info = astro_game.pool_info.load(&deps.storage, U64Key::new(1)).unwrap();
    
  assert_eq!(2, pool_info.count_players);
  assert_eq!( vec![Addr::unchecked("player1"), Addr::unchecked("player2")], pool_info.players);

  //register pool with 



}