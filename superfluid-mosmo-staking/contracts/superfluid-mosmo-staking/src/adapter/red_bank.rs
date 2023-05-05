use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_binary, Addr, Api, QuerierWrapper, QueryRequest, StdResult, Uint128,
    WasmQuery, BankQuery, BalanceResponse, 
};
// use cw20::Balance;
use mars_red_bank_types::{red_bank, red_bank::Market};

#[cw_serde]
pub struct RedBankBase<T>(T);

impl<T> RedBankBase<T> {
    pub fn new(address: T) -> RedBankBase<T> {
        RedBankBase(address)
    }

    pub fn address(&self) -> &T {
        &self.0
    }
}

pub type RedBankUnchecked = RedBankBase<String>;
pub type RedBank = RedBankBase<Addr>;

impl From<RedBank> for RedBankUnchecked {
    fn from(red_bank: RedBank) -> Self {
        Self(red_bank.0.to_string())
    }
}

impl RedBankUnchecked {
    pub fn check(&self, api: &dyn Api) -> StdResult<RedBank> {
        Ok(RedBankBase(api.addr_validate(self.address())?))
    }
}

impl RedBank {
    pub fn query_debt(
        &self,
        querier: &QuerierWrapper,
        user_address: &Addr,
        denom: &str,
    ) -> StdResult<Uint128> {
        let response: red_bank::UserDebtResponse =
            querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
                contract_addr: self.address().to_string(),
                msg: to_binary(&red_bank::QueryMsg::UserDebt {
                    user: user_address.to_string(),
                    denom: denom.to_string(),
                })?,
            }))?;
        Ok(response.amount)
    }

    pub fn query_market(&self, querier: &QuerierWrapper, denom: &str) -> StdResult<Market> {
        querier.query_wasm_smart(
            self.address(),
            &red_bank::QueryMsg::Market {
                denom: denom.to_string(),
            },
        )
    }

    pub fn query_total_collateral_osmo_balance(&self, querier: &QuerierWrapper, denom: &str) -> StdResult<Uint128> {
        let bank_query = BankQuery::Balance {
            address: self.address().into(),
            denom: denom.to_string(),
        };
        
        let balance: BalanceResponse = querier.query(&bank_query.into())?;
        Ok(balance.amount.amount)
    }
}
