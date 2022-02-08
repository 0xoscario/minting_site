use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr,Deps, Order, Coin, Storage, StdResult, Uint128, Decimal};
use cw_storage_plus::{Item, Map, U64Key};
use crate::asset::{Asset, AssetInfo};
pub struct AstroGameContract<'a> {
    pub config: Item<'a, Config>,
    pub prefixed_winner: Map<'a, (&'a [u8], &'a [u8]), Uint128>,
    pub prefixed_players: Map<'a, (&'a [u8], &'a [u8]), Uint128>,
    pub pool_index: Item<'a, u64>,
    pub pool_info: Map<'a, U64Key, PoolInfo>,
}

impl Default for AstroGameContract<'static> {
    fn default() -> Self {
      Self::new(
        "config",
        "prefixed_winner",
        "prefixed_players",
        "pool_index",
        "pool_info"
      )
    }
  }
  
  impl<'a> AstroGameContract<'a> {
    fn new(
      config_key: &'a str,
      prefiexed_winner_key: &'a str,
      prefixed_players_key: &'a str,
      pool_index_key: &'a str,
      pool_info_key: &'a str,
    ) -> Self {
      Self {
        config: Item::new(config_key),
        prefixed_winner: Map::new(prefiexed_winner_key),
        prefixed_players: Map::new(prefixed_players_key),
        pool_index: Item::new(pool_index_key),
        pool_info: Map::new(pool_info_key),
      }
    }
  }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
       pub owner: Addr,
       pub name: String,
       pub global_fee_rate: Decimal,
       
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PoolInfo {
    pub claimed: bool,
    pub is_closed: bool,
    pub amount_reward: Option<Asset>,
    pub support_asset: AssetInfo,
    pub price_per_ticket_to_register: Asset,   
    pub count_players: u64,    
    pub players: Vec<Addr>,
    pub winner: Option<Addr>,

}




// pub const COUNT_PLAYERS: Map<&[u8], Uint128> = Map::new("count");
// pub const COUNT_TICKETS: Map<&[u8], Uint128> = Map::new("tickets");
// pub const COUNT_PREDICTIONS: Map<&[u8], Uint128> = Map::new("bets");
// pub const PREFIXED_WINNER: Map<(&[u8], &[u8]), WinnerRewardClaims> = Map::new("winner");
// pub const PREFIXED_PLAYER: Map<&[u8], Vec<Addr>> = Map::new("players");
// pub const PREFIXED_USER_WINNER_PREDICTION: Map<(&[u8], &[u8]), String> = Map::new("prediction");
// pub const ALL_USER_PREDICTION_WINNER: Map<&[u8], Vec<Addr>> = Map::new("stakers");
// pub const WINNING_PREDICTION: Map<&[u8], String> = Map::new("winning");

// // pub fn predict_winner_save(
// //        storage: &mut dyn Storage,
// //        pool_id: u64,
// //        address: Addr,
// //        prediction_winner: String ,
// //    ) -> StdResult<()> {
// //        let mut exist = true;
// //        // Save combination by senders
// //        PREFIXED_USER_WINNER_PREDICTION.update(
// //            storage,
// //            (&pool_id.to_be_bytes(), address.as_bytes()),
// //            |exists| -> StdResult<String> {
// //                match exists {
// //                    Some(prediction_winner) => {
// //                        let mut modified = prediction_winner;
// //                        Ok(modified)
// //                    }
// //                    None => {
// //                        exist = false;
// //                        Ok(prediction_winner.clone())
// //                    }
// //                }
// //            },
// //        )?;
// //        if !exist {
// //               ALL_USER_PREDICTION_WINNER.update(
// //                storage,
// //                &pool_id.to_be_bytes(),
// //                |exist| -> StdResult<Vec<Addr>> {
// //                    match exist {
// //                        None => Ok(vec![address]),
// //                        Some(players) => {
// //                            let mut data = players;
// //                            data.push(address);
// //                            Ok(data)
// //                        }
// //                    }
// //                },
// //               )?;
// //               COUNT_PREDICTIONS
// //                .update(
// //                    storage,
// //                    &pool_id.to_be_bytes(),
// //                    |exists| -> StdResult<Uint128> {
// //                        match exists {
// //                            None => Ok(Uint128(1)),
// //                            Some(p) => Ok(p.add(Uint128(1))),
// //                        }
// //                    },
// //                )
// //                .map(|_| ())?
// //        }

   
// // }

// pub fn save_player(
//        storage: &mut dyn Storage,
//        pool_id: u64,
//        addr: Addr,
//    ) -> StdResult<()> {
//        PREFIXED_PLAYER.update(
//               storage,
//               &pool_id.to_be_bytes(),
//               |exist| -> StdResult<Vec<Addr>> {
//                   match exist {
//                       None => Ok(vec![address]),
//                       Some(players) => {
//                           let mut data = players;
//                           data.push(address);
//                           Ok(data)
//                       }
//                   }
//               },
//           )?;
//        COUNT_TICKETS
//           .update(
//               storage,
//               &lottery_id.to_be_bytes(),
//               |exists| -> StdResult<Uint128> {
//                   match exists {
//                      None => Ok(Uint128(1)),
//                      Some(p) => Ok(p.add(Uint128(1))),
//                   }
//               },
//           )
//           .map(|_| ())
   
//    }

// pub fn save_winner(
//        storage: &mut dyn Storage,
//        pool_id: u64,
//        addr: Addr,
//        amounts: Uint128,
//    ) -> StdResult<()> {
//        PREFIXED_WINNER.update(
//            storage,
//            (&pool_id.to_be_bytes(), addr.as_bytes()),
//            |exists| -> StdResult<WinnerRewardClaims> {
//                match exists {
//                    None => Ok(WinnerRewardClaims {
//                        claimed: false,
//                        amount: amounts,
//                    }),
//                    Some(claims) => {
//                        let mut amount = claims.amount.add(amounts);
                            
//                        Ok(WinnerRewardClaims {
//                            claimed: false,
//                            amount,
//                        })
//                    }
//                }
//            },
//        )?;
//        Ok(())
   
//    }
   
//  pub fn all_winners(
//        deps: &Deps,
//        pool_id: u64,
//    ) -> StdResult<Vec<(Addr, WinnerRewardClaims)>> {
//        PREFIXED_WINNER
//            .prefix(&pool_id.to_be_bytes())
//            .range(deps.storage, None, None, Order::Ascending)
//            .map(|item| {
//                let (addr, claim) = item?;
//                Ok((Addr::from(addr), claim))
//            })
//            .collect()
//    }
   