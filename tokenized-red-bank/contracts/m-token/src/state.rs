use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};


// NOTE: The data structure might be changed for corresponding_token
pub const CORRESPONDING_TOKEN: Item<String> = Item::new("corresponding_token");
