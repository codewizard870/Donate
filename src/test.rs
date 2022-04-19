use super::*;
use cosmwasm_std::{from_binary, Addr, Coin, Timestamp,
    BankQuery, BalanceResponse, AllBalanceResponse, Uint128, Api};
use cosmwasm_std::testing::{mock_env, mock_info, MOCK_CONTRACT_ADDR};

use crate::contract::{execute, instantiate};
use crate::query::{query};
use crate::msg::{QueryMsg, ExecuteMsg, InstantiateMsg};
use crate::mock_querier::{mock_dependencies};
// use terraswap::asset::{Asset, AssetInfo};
// use terraswap::pair::ExecuteMsg as TerraswapExecuteMsg;

#[test]
fn workflow_luna(){
    let mut deps = mock_dependencies(&[]);
    let mut env = mock_env();
    let mut info = mock_info("owner", &[]);
    const MONTH: u64 = 2592000; //60 * 60 * 24 * 30;
    let mut seconds = 0;
    env.block.time = Timestamp::from_seconds(seconds.clone());

}

