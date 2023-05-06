use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    // Transfer blocker are for the addresses that can execute 
    // IncreaseBlockedTransferAmount and DecreaseBlockedTransferAmount
    // that manages the lock up without the transfer of the mTokens
    pub usability_controler: Vec<String>,
}

#[cw_serde]
pub struct UnusableAmount {
    pub unusable_amount: Uint128,    
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const UNUSABLE_AMOUNT: Map<&Addr, UnusableAmount> = Map::new("unusable_amount");

// NOTE: The data structure might be changed for corresponding_token
pub const CORRESPONDING_TOKEN: Item<String> = Item::new("corresponding_token");
