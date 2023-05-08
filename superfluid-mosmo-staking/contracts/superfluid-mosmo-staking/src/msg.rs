use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Decimal, Uint128, Coin, Addr};
use mars_owner::OwnerUpdate;
use crate::adapter::red_bank::RedBankUnchecked;

#[cw_serde]
pub struct InstantiateMsg {
    /// Contract's owner
    pub owner: String,
    // mOsmo cw20 contract address
    pub mosmo_token: String,
    /// The Mars Protocol money market contract
    pub red_bank: RedBankUnchecked,
    // The ratio that decides how much the unused collateral can be staked
    pub staking_collateral_ratio: Decimal,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Bond will bond all staking tokens sent with the message and release derivative tokens
    /// NOTE: In the future, we may allow bonding to a specific validator
    Bond { amount: Uint128 },
    /// Unbond will "burn" the given amount of derivative tokens and send the unbonded
    /// staking tokens to the message sender (after exit tax is deducted)
    Unbond { amount: Uint128 },
    /// Claim is used to claim your native tokens that you previously "unbonded"
    /// after the chain-defined waiting period (eg. 3 weeks)
    Claim {},

    // TODO: Add WithdrawReward msg to enable users to withdraw their staking rewards
    // WithdrawReward {},

    // Reinvest will check for all accumulated rewards, withdraw them, and
    // re-bond them to the same validator. Anyone can call this, which updates
    // the value of the token (how much under custody).
    // Reinvest {},

    // TODO: add update config msg
    // UpdateConfig {
    //     config: CreateOrUpdateConfig,
    // },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Claims shows the number of tokens this address can access when they are done unbonding
    // #[returns(ClaimsResponse)]
    // Claims { address: String },
    /// Investment shows metadata on the staking info of the contract
    #[returns(BondingStateResponse)]
    BondingState {},
}


#[cw_serde]
pub struct BondingStateResponse {
    pub staked_tokens: Coin,

    /// owner created the contract and takes a cut
    pub owner: Option<String>,
    /// All tokens are bonded to this validator
    pub validator: String,
    /// This is the minimum amount we will pull out to reinvest, as well as a minimum
    /// that can be unbonded (to avoid needless staking tx)
    pub min_withdrawal: Uint128,
}
