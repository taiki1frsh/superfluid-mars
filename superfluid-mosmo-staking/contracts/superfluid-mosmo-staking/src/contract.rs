#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Decimal, DepsMut, Env, MessageInfo, Response,
};

use cw2::set_contract_version;
use cw20_base::contract::{
    execute_burn, execute_mint, execute_send, execute_transfer, query_balance, query_token_info,
};
use cw20_base::state::{MinterData, TokenInfo, TOKEN_INFO};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, BondingStateResponse, QueryMsg};
use crate::state::{Config, CONFIG, OSMO_DENOM};
use crate::execute::{execute_bond, execute_unbond, execute_claim};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:superfluid-mosmo-staking";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        owner: info.sender.to_string(),
        mosmo_token: msg.mosmo_token,
        staking_collateral_ratio: msg.staking_collateral_ratio,
        red_bank: msg.red_bank.check(deps.api)?,
        osmo_token_denom: OSMO_DENOM.to_string(),
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Bond { amount } => execute_bond(deps, env, info),
        ExecuteMsg::Unbond { amount } => execute_unbond(deps, env, info, amount),
        ExecuteMsg::Claim {} => execute_claim(deps, env, info),
    }
}