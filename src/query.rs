#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, Env, StdResult, Uint128
};

use crate::msg::{QueryMsg};
use crate::state::{ TOTAL_DONATED };

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetTotalDonated{ } => {
            to_binary(&TOTAL_DONATED.load(deps.storage).unwrap())
        },
    }
}