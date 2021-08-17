#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(non_snake_case)]

extern crate alloc;

use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::String,
};
use core::convert::TryInto;
use std::ops::{Add, Sub};
use contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use types::{ApiError, CLType, CLTyped, CLValue, ContractHash, Group, Parameter, PublicKey, RuntimeArgs, U256, URef, account::AccountHash, bytesrepr::{FromBytes, ToBytes}, contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys}, runtime_args};

pub enum Error {
    CannotStakeZero = 0,
    CannotWithdrawZero = 1,
    ProvidedRewardTooHigh = 2,
    CannotWithdrawTheStakingToken = 3,
    PreviousRewardsPeriodMustBeComplete = 4,
    CallerIsNotRewardsDistributionContract = 5,
    ZeroHash = 6,
    ContractIsPaused = 7,
    NotNominated = 8,
    OnlyOwner = 9,
    ReentrantCall = 10
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

#[no_mangle]
pub extern "C" fn total_supply() {
    let val: U256 = get_key("total_supply");
    ret(val)
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let owner: AccountHash = runtime::get_named_arg("owner");
    let val: U256 = get_key(&balance_key(&owner));
    ret(val)
}

#[no_mangle]
pub extern "C" fn user_reward_per_token_paid() {
    let owner: AccountHash = runtime::get_named_arg("owner");
    let val: U256 = get_key(&user_reward_per_token_paid_key(&owner));
    ret(val)
}

#[no_mangle]
pub extern "C" fn rewards() {
    let owner: AccountHash = runtime::get_named_arg("owner");
    let val: U256 = get_key(&reward_key(&owner));
    ret(val)
}

#[no_mangle]
pub extern "C" fn call() {
    let owner: AccountHash = runtime::get_named_arg("_owner");
    let nominated_owner: AccountHash = AccountHash::from_bytes(&[0u8; 32]).unwrap().0;
    let rewards_distribution: ContractHash = runtime::get_named_arg("_rewards_distribution");
    let rewards_token: ContractHash = runtime::get_named_arg("_rewards_token");
    let staking_token: ContractHash = runtime::get_named_arg("_staking_token");

    let mut named_keys = NamedKeys::new();
    named_keys.insert(
        "rewards_token".to_string(),
        storage::new_uref(rewards_token).into(),
    );
    named_keys.insert(
        "staking_token".to_string(),
        storage::new_uref(staking_token).into(),
    );
    named_keys.insert(
        "period_finish".to_string(),
        storage::new_uref(U256::from(0)).into(),
    );
    named_keys.insert(
        "reward_rate".to_string(),
        storage::new_uref(U256::from(0)).into(),
    );
    named_keys.insert(
        "rewards_duration".to_string(),
        storage::new_uref(U256::from(604800)).into(),
    );
    named_keys.insert(
        "last_update_time".to_string(),
        storage::new_uref(U256::from(0)).into(),
    );
    named_keys.insert(
        "reward_per_token_stored".to_string(),
        storage::new_uref(U256::from(0)).into(),
    );
    named_keys.insert(
        user_reward_per_token_paid_key(&runtime::get_caller()),
        storage::new_uref(U256::from(0)).into(),
    );
    named_keys.insert(
        reward_key(&runtime::get_caller()),
        storage::new_uref(U256::from(0)).into(),
    );
    named_keys.insert(
        "total_supply".to_string(),
        storage::new_uref(U256::from(0)).into(),
    );
    // from RewardsDistributionRecipient.sol
    named_keys.insert(
        "rewards_distribution".to_string(),
        storage::new_uref(rewards_distribution).into(),
    );
    // from Pausable.sol
    named_keys.insert(
        "last_pause_time".to_string(),
        storage::new_uref(U256::from(0)).into(),
    );
    named_keys.insert(
        "paused".to_string(),
        storage::new_uref(false).into(),
    );
    // from Owned.sol
    named_keys.insert(
        "owner".to_string(),
        storage::new_uref(owner).into(),
    );
    named_keys.insert(
        "nominated_owner".to_string(),
        storage::new_uref(nominated_owner).into(),
    );
    // from ReentrancyGuard.sol
    named_keys.insert(
        "status".to_string(),
        storage::new_uref(U256::from(1)).into(),
    );

    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(endpoint("rewards_token", vec![], ContractHash::cl_type()));
    entry_points.add_entry_point(endpoint("staking_token", vec![], ContractHash::cl_type()));
    entry_points.add_entry_point(endpoint("period_finish", vec![], CLType::U256));
    entry_points.add_entry_point(endpoint("reward_rate", vec![], CLType::U256));
    entry_points.add_entry_point(endpoint("rewards_duration", vec![], CLType::U256));
    entry_points.add_entry_point(endpoint("last_update_time", vec![], CLType::U256));
    entry_points.add_entry_point(endpoint("reward_per_token_stored", vec![], CLType::U256));
    entry_points.add_entry_point(endpoint("rewards_distribution", vec![], ContractHash::cl_type()));
    entry_points.add_entry_point(endpoint("last_pause_time", vec![], CLType::U256));
    entry_points.add_entry_point(endpoint("paused", vec![], CLType::Bool));
    entry_points.add_entry_point(endpoint("owner", vec![], AccountHash::cl_type()));
    entry_points.add_entry_point(endpoint("nominated_owner", vec![], AccountHash::cl_type()));
    entry_points.add_entry_point(endpoint(
        "balance_of",
        vec![Parameter::new("account", AccountHash::cl_type())],
        CLType::U256,
    ));
    entry_points.add_entry_point(endpoint(
        "user_reward_per_token_paid",
        vec![Parameter::new("account", AccountHash::cl_type())],
        CLType::U256,
    ));
    entry_points.add_entry_point(endpoint(
        "rewards",
        vec![Parameter::new("account", AccountHash::cl_type())],
        CLType::U256,
    ));

    let (contract_hash, _) =
        storage::new_locked_contract(entry_points, Some(named_keys), None, None);
    runtime::put_key("StakingRewards", contract_hash.into());
    runtime::put_key("StakingRewards_hash", storage::new_uref(contract_hash).into());
}

fn ret<T: CLTyped + ToBytes>(value: T) {
    runtime::ret(CLValue::from_t(value).unwrap_or_revert())
}

fn get_key<T: FromBytes + CLTyped + Default>(name: &str) -> T {
    match runtime::get_key(name) {
        None => Default::default(),
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            storage::read(key).unwrap_or_revert().unwrap_or_revert()
        }
    }
}

fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}

fn user_reward_per_token_paid_key(account: &AccountHash) -> String {
    format!("user_reward_per_token_paid_{}", account)
}

fn reward_key(account: &AccountHash) -> String {
    format!("rewards_{}", account)
}

fn balance_key(account: &AccountHash) -> String {
    format!("balances_{}", account)
}

fn endpoint(name: &str, param: Vec<Parameter>, ret: CLType) -> EntryPoint {
    EntryPoint::new(
        String::from(name),
        param,
        ret,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}