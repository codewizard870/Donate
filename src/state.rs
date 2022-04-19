use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Uint128, Coin, StdResult, DepsMut};
use cw_storage_plus::{Item, Map, U128Key};

pub const TREASURY: Item<Addr> = Item::new("treasury");
pub const TOTAL_DONATED: Item<Uint128> = Item::new("donated");