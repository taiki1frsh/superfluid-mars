use std::ops::{Add, Mul};

use cosmwasm_std::{
    to_binary, DepsMut, Empty, Env, MessageInfo, QueryRequest, Response, WasmQuery, Uint128, Addr, coin, QuerierWrapper, Decimal, StdError,
};
use mars_red_bank::interest_rates::get_underlying_debt_amount;

use crate::error::ContractError;
use crate::state::{Config, CONFIG};
use crate::adapter::red_bank::{RedBank, RedBankUnchecked};

pub fn execute_bond(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // TODO: Query the utilization ratio of the Osmo in Mars red-bank
    let utilization_ratio = current_osmo_utilization_rate(deps, env)?;


    Ok(Response::default())
}

pub fn execute_unbond(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

pub fn execute_claim(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

// Returns the current utilization rate of the osmo token in red-bank
// between 0 and 1 in Decimal type
pub fn current_osmo_utilization_rate(
    deps: DepsMut,
    env: Env,
) -> Result<Decimal, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    // let osmo_token = deps.api.addr_humanize(&config.mosmo_token)?;

    let red_bank = config.clone().red_bank;

    let total_osmo_debt_amount = current_osmo_total_debt(&deps.querier, env.block.time.seconds(), &config, &red_bank)?;
    let balance_of_osmo_in_red_bank = red_bank.query_total_collateral_osmo_balance(
        &deps.querier,
        &config.osmo_token_denom,
    )?;
    
    // current utilization ratio = total_debt / (contract_current_balance + total_debt)
    let current_utilization_rate = if !total_osmo_debt_amount.is_zero() {
        let liquidity_and_debt = balance_of_osmo_in_red_bank
            .checked_add(total_osmo_debt_amount)
            .map_err(|err| ContractError::Std(cosmwasm_std::StdError::Overflow { source: err }))?;
        Decimal::from_ratio(total_osmo_debt_amount, liquidity_and_debt)
    } else {
        Decimal::zero()
    };

    Ok(current_utilization_rate)
}

pub fn current_osmo_total_debt(
    querier: &QuerierWrapper,
    timestamp: u64,
    config: &Config,
    red_bank: &RedBank,
) -> Result<Uint128, ContractError> {
    let market = red_bank.query_market(querier, &config.osmo_token_denom)?;
    let total_debt_amount =
        get_underlying_debt_amount(market.debt_total_scaled, &market, timestamp)?;
    
    Ok(total_debt_amount)
}
