#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(non_snake_case)]

extern crate alloc;

use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::String,
};
use core::convert::TryInto;
use std::{ops::{Add, Div, Mul, Sub}, time::SystemTime};
use contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use types::{ApiError, CLType, CLTyped, CLValue, ContractHash, Group, Parameter, PublicKey, RuntimeArgs, U256, URef, account::AccountHash, bytesrepr::{FromBytes, ToBytes}, contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys}, runtime_args};
use libs::math;

pub enum Error {
    CannotStakeZero = 0,
    CannotWithdrawZero = 1,
    ProvidedRewardTooHigh = 2,
    CannotWithdrawTheStakingToken = 3,
    InCompletePreviousRewardsPeriod = 4,
    OnlyRewardsDistributionContract = 5,
    ZeroHash = 6,
    ContractIsPaused = 7,
    OnlyNominatedOwner = 8,
    OnlyOwner = 9,
    ReentrantCall = 10
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

/* ✖✖✖✖✖✖✖✖✖✖✖ Public getters - Start ✖✖✖✖✖✖✖✖✖✖✖ */
#[no_mangle]
pub extern "C" fn rewards_token() {
    let val: ContractHash = get_key("rewards_token");
    ret(val)
}

#[no_mangle]
pub extern "C" fn staking_token() {
    let val: ContractHash = get_key("staking_token");
    ret(val)
}

#[no_mangle]
pub extern "C" fn period_finish() {
    let val: U256 = get_key("period_finish");
    ret(val)
}

#[no_mangle]
pub extern "C" fn reward_rate() {
    let val: U256 = get_key("reward_rate");
    ret(val)
}

#[no_mangle]
pub extern "C" fn rewards_duration() {
    let val: U256 = get_key("rewards_duration");
    ret(val)
}

#[no_mangle]
pub extern "C" fn last_update_time() {
    let val: U256 = get_key("last_update_time");
    ret(val)
}

#[no_mangle]
pub extern "C" fn reward_per_token_stored() {
    let val: U256 = get_key("reward_per_token_stored");
    ret(val)
}

#[no_mangle]
pub extern "C" fn user_reward_per_token_paid() {
    let owner: AccountHash = runtime::get_named_arg("owner");
    let val: U256 = get_key(&user_reward_per_token_paid_key(&owner));
    ret(val)
}

#[no_mangle]
pub extern "C" fn reward_of() {
    let owner: AccountHash = runtime::get_named_arg("owner");
    let val: U256 = get_key(&reward_key(&owner));
    ret(val)
}

// from RewardsDistributionRecipient.sol
#[no_mangle]
pub extern "C" fn rewards_distribution() {
    let val: ContractHash = get_key("rewards_distribution");
    ret(val)
}

// from Pausable.sol
#[no_mangle]
pub extern "C" fn last_pause_time() {
    let val: U256 = get_key("last_pause_time");
    ret(val)
}

#[no_mangle]
pub extern "C" fn paused() {
    let val: bool = get_key("paused");
    ret(val)
}

// from Owned.sol
#[no_mangle]
pub extern "C" fn owner() {
    let val: AccountHash = get_key("owner");
    ret(val)
}

#[no_mangle]
pub extern "C" fn nominated_owner() {
    let val: AccountHash = get_key("nominated_owner");
    ret(val)
}
/* ✖✖✖✖✖✖✖✖✖✖✖ Public getters - End ✖✖✖✖✖✖✖✖✖✖✖ */

/* ✖✖✖✖✖✖✖✖✖✖✖ External functions - Start ✖✖✖✖✖✖✖✖✖✖✖ */

// from RewardsDistributionRecipient.sol
#[no_mangle]
pub extern "C" fn set_rewards_distribution() {
    _only_owner();
    let rewards_distribution: ContractHash = runtime::get_named_arg("rewards_distribution");
    set_key("rewards_distribution", rewards_distribution);
}
// from Pausable.sol
#[no_mangle]
pub extern "C" fn set_paused() {
    _only_owner();
    let paused: bool = runtime::get_named_arg("paused");
    // Ensure we're actually changing the state before we do anything
    if (paused == get_key::<bool>("paused")) {
        return;
    }
    // Set our paused state
    set_key("paused", paused);
    // If applicable, set the last pause time
    if (paused) {
        set_key("last_pause_time", U256::from(SystemTime::now().elapsed().unwrap().as_secs()));
    }
}
// from Owned.sol
#[no_mangle]
pub extern "C" fn nominate_new_owner() {
    _only_owner();
    let owner: AccountHash = runtime::get_named_arg("owner");
    set_key("nominated_owner", owner);
}
#[no_mangle]
pub extern "C" fn accept_ownership() {
    _only_nominated_owner();
    set_key("owner", get_key::<AccountHash>("nominated_owner"));
    set_key("nominated_owner", AccountHash::from_bytes(&[0u8; 32]).unwrap().0);
}
// from StakingRewards.sol
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
pub extern "C" fn get_reward_for_duration() {
    let val: U256 = get_key::<U256>("reward_rate").mul(get_key::<U256>("rewards_duration"));
    ret(val)
}

/* ✖✖✖✖✖✖✖✖✖✖✖ External functions - End ✖✖✖✖✖✖✖✖✖✖✖ */

/* ✖✖✖✖✖✖✖✖✖✖✖ Public functions - Start ✖✖✖✖✖✖✖✖✖✖✖ */
// from StakingRewards.sol
pub fn last_time_reward_applicable() -> U256 {
    math::min(
        U256::from(&runtime::get_blocktime().to_bytes().unwrap()[..]),
        get_key::<U256>("period_finish")
    )
}

pub fn reward_per_token() -> U256 {
    let total_supply: U256 = get_key("total_supply");
    let reward_per_token_stored: U256 = get_key("reward_per_token_stored");
    if (total_supply == U256::from(0)) {
        return reward_per_token_stored;
    }
    let last_update_time: U256 = get_key("last_update_time");
    let reward_rate: U256 = get_key("reward_rate");
    return reward_per_token_stored
    .add(
        last_time_reward_applicable()
        .sub(last_update_time)
        .mul(reward_rate)
        .mul(U256::from(10u8.pow(18)))
        .div(total_supply)
    );
}

pub fn earned(account: AccountHash) -> U256 {
    get_key::<U256>(&balance_key(&account))
    .mul(
        reward_per_token()
        .sub(get_key::<U256>(&user_reward_per_token_paid_key(&account)))
    )
    .div(U256::from(10u8.pow(18)))
    .add(get_key::<U256>(&reward_key(&account)))
}

/* ✖✖✖✖✖✖✖✖✖✖✖ Public functions - End ✖✖✖✖✖✖✖✖✖✖✖ */

#[no_mangle]
pub extern "C" fn call() {
    let owner: AccountHash = runtime::get_named_arg("owner");
    let nominated_owner: AccountHash = AccountHash::from_bytes(&[0u8; 32]).unwrap().0;
    let rewards_distribution: ContractHash = runtime::get_named_arg("rewards_distribution");
    let rewards_token: ContractHash = runtime::get_named_arg("rewards_token");
    let staking_token: ContractHash = runtime::get_named_arg("staking_token");

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
    // _entered and _not_entered are constants, so we don't need to use them
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
    entry_points.add_entry_point(endpoint("total_supply", vec![], CLType::U256));
    entry_points.add_entry_point(endpoint("get_reward_for_duration", vec![], CLType::U256));
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
        "reward_of",
        vec![Parameter::new("account", AccountHash::cl_type())],
        CLType::U256,
    ));
    // from RewardsDistributionRecipient.sol
    entry_points.add_entry_point(endpoint("rewards_distribution", vec![], ContractHash::cl_type()));
    entry_points.add_entry_point(endpoint(
        "set_rewards_distribution", 
        vec![Parameter::new("rewards_distribution", ContractHash::cl_type())], 
        CLType::Unit
    ));
    // from Pausable.sol
    entry_points.add_entry_point(endpoint("last_pause_time", vec![], CLType::U256));
    entry_points.add_entry_point(endpoint("paused", vec![], CLType::Bool));
    entry_points.add_entry_point(endpoint(
        "set_paused", 
        vec![Parameter::new("paused", CLType::Bool)], 
        CLType::Unit
    ));
    // from Owned.sol
    entry_points.add_entry_point(endpoint("owner", vec![], AccountHash::cl_type()));
    entry_points.add_entry_point(endpoint("nominated_owner", vec![], AccountHash::cl_type()));
    entry_points.add_entry_point(endpoint(
        "nominate_new_owner", 
        vec![Parameter::new("owner", ContractHash::cl_type())], 
        CLType::Unit
    ));
    entry_points.add_entry_point(endpoint("accept_ownership", vec![], CLType::Unit));

    let (contract_hash, _) =
        storage::new_locked_contract(entry_points, Some(named_keys), None, None);
    runtime::put_key("StakingRewards", contract_hash.into());
    runtime::put_key("StakingRewards_hash", storage::new_uref(contract_hash).into());
}

/* ✖✖✖✖✖✖✖✖✖✖✖ Internal Functions - Start ✖✖✖✖✖✖✖✖✖✖✖ */
fn _only_owner() {
    if (runtime::get_caller() != get_key::<AccountHash>("owner")) {
        runtime::revert(Error::OnlyOwner);
    }
}

fn _only_nominated_owner() {
    if (runtime::get_caller() != get_key::<AccountHash>("nominated_owner")) {
        runtime::revert(Error::OnlyNominatedOwner);
    }
}

fn _not_paused() {
    if (get_key::<bool>("paused")) {
        runtime::revert(Error::ContractIsPaused);
    }
}

fn _only_rewards_distribution() {
    if (runtime::get_caller().value() != get_key::<ContractHash>("rewards_distribution").value()) {
        runtime::revert(Error::OnlyRewardsDistributionContract);
    }
}

fn _update_reward(account: AccountHash) {
    //set_key("reward_per_token_stored", reward_per_token());
    set_key("reward_per_token_stored", U256::from(0));
    set_key("last_update_time", last_time_reward_applicable());
    if (account != AccountHash::from_bytes(&[0u8; 32]).unwrap().0) {
        set_key(&reward_key(&account), earned(account));
        set_key(&user_reward_per_token_paid_key(&account), get_key::<U256>("reward_per_token_stored"));
    }
}

fn _non_reentrant() {
    if (get_key::<U256>("status") == U256::from(2)) {
        runtime::revert(Error::ReentrantCall);
    }
    set_key("status", U256::from(2));
}

fn _free_entrancy() {
    set_key("status", U256::from(1));
}
/* ✖✖✖✖✖✖✖✖✖✖✖ Internal Functions - End ✖✖✖✖✖✖✖✖✖✖✖ */

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