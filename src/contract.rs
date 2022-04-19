#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{
    Addr, DepsMut, Env, MessageInfo, Response,
    Uint128, BankMsg
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{ TREASURY, TOTAL_DONATED };

// version info for migration info
const CONTRACT_NAME: &str = "DONATE";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    TREASURY.save(deps.storage, &msg.treasury)?;
    TOTAL_DONATED.save(deps.storage, &Uint128::zero())?;

    Ok(Response::new()
        .add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Donate{ }
            => donate(deps, info),
    }
}

pub fn donate(
    deps:DepsMut, 
    info:MessageInfo, 
)
    -> Result<Response, ContractError>
{
    let mut total = TOTAL_DONATED.load(deps.storage)?;
    total += info.funds[0].amount;
    TOTAL_DONATED.save(deps.storage, &total)?;
    
    let msg = BankMsg::Send {
        to_address: TREASURY.load(deps.storage)?.to_string(),
        amount: info.funds
    };

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "SetConfig"))                                
}
