use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    // Contract's owner
    pub owner: String,
    // mOsmo cw20 contract address
    pub mosmo_token: String,
    // The ratio that decides how much the unused collateral can be staked
    pub staking_collateral_ratio: u8,
    // TODO: set the minimum bonding amount
    // pub min_bonding_amount: Uint128
    // TODO: set the minimum reward claim amount
    // pub min_reward_claim_amount: Uint128
}

pub const CONFIG: Item<Config> = Item::new("config");
