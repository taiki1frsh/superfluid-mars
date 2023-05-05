#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::Order::Ascending;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128,
};

use cw2::set_contract_version;
use cw20::{
    BalanceResponse, Cw20Coin, Cw20ReceiveMsg, DownloadLogoResponse, EmbeddedLogo, Logo, LogoInfo,
    MarketingInfoResponse, MinterResponse, TokenInfoResponse,
};
use cw20_base::contract::{
    query_balance, query_token_info, query_minter,
};
use cw20_base::allowances::{query_allowance};
use cw20_base::enumerable::{query_owner_allowances, query_spender_allowances, query_all_accounts};
use cw20_base::state::{TokenInfo, MinterData, TOKEN_INFO, BALANCES};

use crate::msg::{ExecuteMsg, QueryMsg, InstantiateMsg};
use crate::error::ContractError;
use crate::execute::{
    transfer, burn, mint, send, increase_allowance, decrease_allowance,
    transfer_from, burn_from, send_from,
};
use crate::state::CORRESPONDING_TOKEN;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:m-token";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn instantiate(
    mut deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
  // check valid token info
  msg.validate()?;
  // create initial accounts
  let total_supply = create_accounts(&mut deps, &msg.initial_balances)?;

  if let Some(limit) = msg.get_cap() {
      if total_supply > limit {
          return Err(StdError::generic_err("Initial supply greater than cap").into());
      }
  }

  let mint = match msg.mint {
      Some(m) => Some(MinterData {
          minter: deps.api.addr_validate(&m.minter)?,
          cap: m.cap,
      }),
      None => None,
  };

  // store token info
  let data = TokenInfo {
      name: msg.name,
      symbol: msg.symbol,
      decimals: msg.decimals,
      total_supply,
      mint,
  };
  TOKEN_INFO.save(deps.storage, &data)?;

  // save corresponding token information 
  CORRESPONDING_TOKEN.save(deps.storage, &msg.corresponding_token)?;

  Ok(Response::default())
}

pub fn create_accounts(
    deps: &mut DepsMut,
    accounts: &[Cw20Coin],
) -> Result<Uint128, ContractError> {
    validate_accounts(accounts)?;

    let mut total_supply = Uint128::zero();
    for row in accounts {
        let address = deps.api.addr_validate(&row.address)?;
        BALANCES.save(deps.storage, &address, &row.amount)?;
        total_supply += row.amount;
    }

    Ok(total_supply)
}

pub fn validate_accounts(accounts: &[Cw20Coin]) -> Result<(), ContractError> {
    let mut addresses = accounts.iter().map(|c| &c.address).collect::<Vec<_>>();
    addresses.sort();
    addresses.dedup();

    if addresses.len() != accounts.len() {
        Err(ContractError::DuplicateInitialBalanceAddresses {})
    } else {
        Ok(())
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Transfer{ recipient, amount } => {
            transfer(deps, env, info, recipient, amount)
        }
        ExecuteMsg::Burn { amount } => burn(deps, env, info, amount),
        ExecuteMsg::Send {
            contract,
            amount,
            msg,
        } => send(deps, env, info, contract, amount, msg),
        ExecuteMsg::Mint { recipient, amount } => mint(deps, env, info, recipient, amount),
        ExecuteMsg::IncreaseAllowance {
            spender,
            amount,
            expires,
        } => increase_allowance(deps, env, info, spender, amount, expires),
        ExecuteMsg::DecreaseAllowance {
            spender,
            amount,
            expires,
        } => decrease_allowance(deps, env, info, spender, amount, expires),
        ExecuteMsg::TransferFrom {
            owner,
            recipient,
            amount,
        } => transfer_from(deps, env, info, owner, recipient, amount),
        ExecuteMsg::BurnFrom { owner, amount } => burn_from(deps, env, info, owner, amount),
        ExecuteMsg::SendFrom {
            owner,
            contract,
            amount,
            msg,
        } => send_from(deps, env, info, owner, contract, amount, msg),
        // ExecuteMsg::Rebalance {} => rebalance(deps, env, info),
        // ExecuteMsg::TransferOnLiquidation{ recipient, amount } => {
        //     transfer_on_liquidation(deps, env, info, recipient, amount)
        // }
    }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Balance { address } => to_binary(&query_balance(deps, address)?),
        QueryMsg::TokenInfo {} => to_binary(&query_token_info(deps)?),
        QueryMsg::Minter {} => to_binary(&query_minter(deps)?),
        QueryMsg::Allowance { owner, spender } => {
            to_binary(&query_allowance(deps, owner, spender)?)
        }
        QueryMsg::AllAllowances {
            owner,
            start_after,
            limit,
        } => to_binary(&query_owner_allowances(deps, owner, start_after, limit)?),
        QueryMsg::AllSpenderAllowances {
            spender,
            start_after,
            limit,
        } => to_binary(&query_spender_allowances(
            deps,
            spender,
            start_after,
            limit,
        )?),
        QueryMsg::AllAccounts { start_after, limit } => {
            to_binary(&query_all_accounts(deps, start_after, limit)?)
        }
    }
}


