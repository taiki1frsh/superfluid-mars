use cosmwasm_std::{
    Addr, Decimal, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128, Binary,
};
use cw_utils::Expiration;
// use mars_owner::{OwnerError, OwnerInit::SetInitialOwner, OwnerUpdate};
// use mars_red_bank_types::{
//     address_provider::{self, MarsAddressType},
//     error::MarsError,
//     red_bank::{
//         Config, CreateOrUpdateConfig, Debt, InitOrUpdateAssetParams, InstantiateMsg, Market,
//     },
// };

use cw20_base::allowances::{
    execute_burn_from, execute_decrease_allowance, execute_increase_allowance, execute_send_from,
    execute_transfer_from, query_allowance,
};
use cw20_base::contract::{
    execute_burn, execute_mint, execute_send, execute_transfer, query_balance, query_token_info,
};
use cw20_base::state::{MinterData, TokenInfo, TOKEN_INFO};

use crate::error::ContractError;

// Wrap basic exectuion function from cw20_base
// TODO: Insert the validation logic to avoid  collateralization constraints violation.
pub fn transfer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // TODO: Validate if the free token amount is enough to cover the transfer amount
    // based on UnusableAmount

    // return after the execute_transfe call
    Ok(execute_transfer(deps, env, info, recipient, amount)?)
}

// TODO: Insert the validation logic to check if buner address is the owner of Mars.
pub fn burn(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {

    // return after the execute_burn call
    Ok(execute_burn(deps, env, info, amount)?)
}

pub fn mint(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    

    // return after the execute_mint call
    Ok(execute_mint(deps, env, info, recipient, amount)?)
}

pub fn send(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    contract: String,
    amount: Uint128,
    msg: Binary,
) -> Result<Response, ContractError> {
    // TODO: Validate if the free token amount is enough to cover the transfer amount
    // based on UnusableAmount

    // return after the execute_send call
    Ok(execute_send(deps, _env, info, contract, amount, msg)?)
}

pub fn increase_allowance(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    spender: String,
    amount: Uint128,
    expires: Option<Expiration>,
) -> Result<Response, ContractError> {
    // TODO: Validate if the free token amount is enough to cover the transfer amount
    // based on UnusableAmount

    // return after the execute_increase_allowance call
    Ok(execute_increase_allowance(deps, env, info, spender, amount, expires)?)
}

pub fn decrease_allowance(    
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    spender: String,
    amount: Uint128,
    expires: Option<Expiration>,
) -> Result<Response, ContractError> {

    // return after the execute_decrease_allowance call
    Ok(execute_decrease_allowance(deps, env, info, spender, amount, expires)?)
}

pub fn transfer_from(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    owner: String,
    recipient: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // TODO: Validate if the free token amount is enough to cover the transfer amount
    // based on UnusableAmount

    // return after the execute_transfer_from call
    Ok(execute_transfer_from(deps, env, info, owner, recipient, amount)?)
}

pub fn burn_from(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    owner: String,
    amount: Uint128,
) -> Result<Response, ContractError> {

    // return after the execute_burn_from call
    Ok(execute_burn_from(deps, env, info, owner, amount)?)
}

pub fn send_from(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    owner: String,
    contract: String,
    amount: Uint128,
    msg: Binary,
) -> Result<Response, ContractError> {
    // TODO: Validate if the free token amount is enough to cover the transfer amount
    // based on UnusableAmount

    // return after the execute_send_from call
    Ok(execute_send_from(deps, _env, info, owner, contract, amount, msg)?)
}