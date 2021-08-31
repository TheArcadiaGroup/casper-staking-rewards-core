#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(non_snake_case)]

extern crate alloc;

use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::String,
};
use core::convert::TryInto;
use std::ops::{Add, Div, Mul, Sub};
use contract::{contract_api::{runtime::{self, call_contract}, storage::{self, create_contract_package_at_hash}}, unwrap_or_revert::UnwrapOrRevert};
use types::{ApiError, BlockTime, CLType, CLTyped, CLValue, Contract, ContractHash, Group, Key, Parameter, PublicKey, RuntimeArgs, U256, URef, account::AccountHash, bytesrepr::{FromBytes, ToBytes}, contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys}, runtime_args, system::CallStackElement};
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
    let val: ContractHash = get_key("staking_rewards_data", "rewards_token");
    ret(val)
}

#[no_mangle]
pub extern "C" fn staking_token() {
    let val: ContractHash = get_key("staking_rewards_data", "staking_token");
    ret(val)
}

#[no_mangle]
pub extern "C" fn period_finish() {
    let val: U256 = get_key("staking_rewards_data", "period_finish");
    ret(val)
}

#[no_mangle]
pub extern "C" fn reward_rate() {
    let val: U256 = get_key("staking_rewards_data", "reward_rate");
    ret(val)
}

#[no_mangle]
pub extern "C" fn rewards_duration() {
    let val: U256 = get_key("staking_rewards_data", "rewards_duration");
    ret(val)
}

#[no_mangle]
pub extern "C" fn last_update_time() {
    let val: U256 = get_key("staking_rewards_data", "last_update_time");
    ret(val)
}

#[no_mangle]
pub extern "C" fn reward_per_token_stored() {
    let val: U256 = get_key("staking_rewards_data", "reward_per_token_stored");
    ret(val)
}

#[no_mangle]
pub extern "C" fn user_reward_per_token_paid() {
    let owner: AccountHash = runtime::get_named_arg("owner");
    let val: U256 = get_key("user_reward_per_token_paid", &owner.to_string());
    ret(val)
}

#[no_mangle]
pub extern "C" fn reward_of() {
    let owner: AccountHash = runtime::get_named_arg("owner");
    let val: U256 = get_key("rewards", &owner.to_string());
    ret(val)
}

#[no_mangle]
pub extern "C" fn total_supply() {
    let val: U256 = get_key("staking_rewards_data", "total_supply");
    ret(val)
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let owner: AccountHash = runtime::get_named_arg("owner");
    let val: U256 = get_key("balances", &owner.to_string());
    ret(val)
}

// from RewardsDistributionRecipient.sol
#[no_mangle]
pub extern "C" fn rewards_distribution() {
    let val: ContractHash = get_key("staking_rewards_data", "rewards_distribution");
    ret(val)
}

// from Pausable.sol
#[no_mangle]
pub extern "C" fn last_pause_time() {
    let val: U256 = get_key("staking_rewards_data", "last_pause_time");
    ret(val)
}

#[no_mangle]
pub extern "C" fn paused() {
    let val: bool = get_key("staking_rewards_data", "paused");
    ret(val)
}

// from Owned.sol
#[no_mangle]
pub extern "C" fn owner() {
    let val: AccountHash = get_key("staking_rewards_data", "owner");
    ret(val)
}

#[no_mangle]
pub extern "C" fn nominated_owner() {
    let val: AccountHash = get_key("staking_rewards_data", "nominated_owner");
    ret(val)
}
/* ✖✖✖✖✖✖✖✖✖✖✖ Public getters - End ✖✖✖✖✖✖✖✖✖✖✖ */

/* ✖✖✖✖✖✖✖✖✖✖✖ External functions - Start ✖✖✖✖✖✖✖✖✖✖✖ */

// from RewardsDistributionRecipient.sol
#[no_mangle]
pub extern "C" fn set_rewards_distribution() {
    _only_owner();
    let rewards_distribution = ContractHash::from(
        runtime::get_named_arg::<Key>("rewards_distribution").into_hash().unwrap_or_revert()
    );
    set_key("staking_rewards_data", "rewards_distribution", rewards_distribution);
}
// from Pausable.sol
#[no_mangle]
pub extern "C" fn set_paused() {
    _only_owner();
    let paused: bool = runtime::get_named_arg("paused");
    // Ensure we're actually changing the state before we do anything
    if (paused != get_key::<bool>("staking_rewards_data", "paused")) {
        // Set our paused state
        set_key("staking_rewards_data", "paused", paused);
        // If applicable, set the last pause time
        if (paused) {
            set_key(
            "staking_rewards_data",
            "last_pause_time",
            U256::from(u64::from(runtime::get_blocktime())));
        }
    }
}
// from Owned.sol
#[no_mangle]
pub extern "C" fn nominate_new_owner() {
    _only_owner();
    let owner: AccountHash = runtime::get_named_arg("owner");
    set_key("staking_rewards_data", "nominated_owner", owner);
}
#[no_mangle]
pub extern "C" fn accept_ownership() {
    _only_nominated_owner();
    set_key(
        "staking_rewards_data",
        "owner",
        get_key::<AccountHash>("staking_rewards_data", "nominated_owner")
    );
    set_key(
        "staking_rewards_data",
        "nominated_owner",
        AccountHash::new([0u8; 32])
    );
}
// from StakingRewards.sol
#[no_mangle]
pub extern "C" fn get_reward_for_duration() {
    let val: U256 = get_key::<U256>("staking_rewards_data", "reward_rate")
    .mul(get_key::<U256>("staking_rewards_data", "rewards_duration"));
    ret(val)
}

/* ========== MUTATIVE FUNCTIONS ========== */
/// Function: stake(amount: U256)
///
/// # Purpose
/// Stake tokens.
/// # Arguments
/// * `amount` - a `U256` which holds the staked token's amount.
#[no_mangle]
pub extern "C" fn stake() {
    _non_reentrant();
    _not_paused();
    _update_reward(runtime::get_caller());
    let amount: U256 = runtime::get_named_arg("amount");
    if (amount <= U256::from(0)) {
        _free_entrancy();
        runtime::revert(Error::CannotStakeZero);
    }
    set_key(
        "staking_rewards_data",
        "total_supply",
        get_key::<U256>("staking_rewards_data", "total_supply").add(amount)
    );
    let old_balance: U256 = get_key("balances", &runtime::get_caller().to_string());
    set_key(
        "balances",
        &runtime::get_caller().to_string(),
        old_balance.add(amount)
    );
    let current_contract_hash = get_key::<ContractHash>("staking_rewards_data", "contract_hash");
    assert_ne!(current_contract_hash, ContractHash::new([0u8; 32]));
    _safe_transfer_from(
        get_key::<ContractHash>("staking_rewards_data", "staking_token"), 
        Key::Account(runtime::get_caller()),
        Key::Hash(current_contract_hash.value()),
        amount
    );
    _free_entrancy();
}

/// Function: exit()
///
/// # Purpose
/// Withdraws an account's staked tokens and gets reward tokens if he has earned any.
#[no_mangle]
pub extern "C" fn exit() {
    withdraw(get_key::<U256>("balances", &runtime::get_caller().to_string()));
    get_reward();
}

/* ========== RESTRICTED FUNCTIONS ========== */
/// Function: notify_reward_amount(reward: U256)
///
/// # Purpose
/// Adds a new reward.
/// # Arguments
/// * `reward` - a `U256` which holds the provided reward amount.
#[no_mangle]
pub extern "C" fn notify_reward_amount() {
    _only_rewards_distribution();
    _update_reward(AccountHash::new([0u8; 32]));
    let reward: U256 = runtime::get_named_arg("reward");
    if (U256::from(u64::from(runtime::get_blocktime())) >= get_key::<U256>("staking_rewards_data", "period_finish")) {
        set_key(
            "staking_rewards_data",
            "reward_rate",
            reward.div(get_key::<U256>("staking_rewards_data", "rewards_duration"))
        );
    }
    else {
        let remaining: U256 = get_key::<U256>("staking_rewards_data", "period_finish").sub(
            U256::from(u64::from(runtime::get_blocktime()))
        );
        let leftover: U256 = remaining.mul(get_key::<U256>("staking_rewards_data", "reward_rate"));
        set_key(
            "staking_rewards_data", 
            "reward_rate", 
            reward.add(leftover).div(get_key::<U256>("staking_rewards_data", "rewards_duration"))
        );
    }
    // Ensure the provided reward amount is not more than the balance in the contract.
    // This keeps the reward rate in the right range, preventing overflows due to
    // very high values of rewardRate in the earned and rewardsPerToken functions;
    // Reward + leftover must be less than 2^256 / 10^18 to avoid overflow.
    // let current_contract_hash: ContractHash = runtime::get_key("StakingRewards")
    //     .and_then(Key::into_hash)
    //     .expect("should have key")
    //     .into();
    let current_contract_hash = get_key::<ContractHash>("staking_rewards_data", "contract_hash");
    let balance: U256 = call_contract(
        get_key::<ContractHash>("staking_rewards_data", "rewards_token"),
        "balance_of",
        runtime_args! {
            "account" => Key::Hash(current_contract_hash.value())
        }
    );
    if (
        get_key::<U256>("staking_rewards_data", "reward_rate") > 
        balance.div(get_key::<U256>("staking_rewards_data", "rewards_duration"))
    ) {
        runtime::revert(Error::ProvidedRewardTooHigh);
    }
    set_key(
        "staking_rewards_data", 
        "last_update_time",
        U256::from(u64::from(runtime::get_blocktime()))
    );
    set_key(
        "staking_rewards_data", 
        "period_finish",
        U256::from(u64::from(runtime::get_blocktime()))
        .add(get_key::<U256>("staking_rewards_data", "rewards_duration"))
    );
}

/// Function: update_period_finish(timestamp: U256)
///
/// # Purpose
/// End rewards emission earlier.
/// # Arguments
/// * `timestamp` - a `U256` which holds the new `period_finish`.
#[no_mangle]
pub extern "C" fn update_period_finish() {
    _only_owner();
    _update_reward(AccountHash::new([0u8; 32]));
    set_key(
        "staking_rewards_data", 
        "period_finish",
        runtime::get_named_arg::<U256>("timestamp")
    );
}

/// Function: recover_erc20(token_contract_hash: String, token_amount: U256)
///
/// # Purpose
/// Added to support recovering LP Rewards from other systems such as BAL to be distributed to holders.
/// # Arguments
/// * `token_contract_hash` - a `String` which holds the token's contract hash.
/// * `token_amount` - a `U256` which holds the token's amount.
#[no_mangle]
pub extern "C" fn recover_erc20() {
    _only_owner();
    let token_contract_hash = ContractHash::from(
        runtime::get_named_arg::<Key>("token_contract_key").into_hash().unwrap_or_revert()
    );
    let token_amount: U256 = runtime::get_named_arg("token_amount");
    if (token_contract_hash == get_key::<ContractHash>("staking_rewards_data", "staking_token")) {
        runtime::revert(Error::CannotWithdrawTheStakingToken);
    }
    _safe_transfer(
        token_contract_hash,
        Key::Account(get_key::<AccountHash>("staking_rewards_data", "owner")),
        token_amount
    );
}

/// Function: set_rewards_duration(rewards_duration: U256)
///
/// # Purpose
/// Sets the `rewards_duration` key.
/// # Arguments
/// * `rewards_duration` - a `U256` which holds the new `rewards_duration`'s value.
#[no_mangle]
pub extern "C" fn set_rewards_duration() {
    _only_owner();
    if (
        U256::from(u64::from(runtime::get_blocktime())) <= 
        get_key::<U256>("staking_rewards_data", "period_finish")
    ) {
        runtime::revert(Error::InCompletePreviousRewardsPeriod);
    }
    set_key(
        "staking_rewards_data",
        "rewards_duration",
        runtime::get_named_arg::<U256>("rewards_duration")
    );
}

/* ✖✖✖✖✖✖✖✖✖✖✖ External functions - End ✖✖✖✖✖✖✖✖✖✖✖ */

/* ✖✖✖✖✖✖✖✖✖✖✖ Public functions - Start ✖✖✖✖✖✖✖✖✖✖✖ */
// from StakingRewards.sol
/// # Purpose
/// Returns the last time the reward was applicable.
/// # Returns
/// * `timestamp` - an `U256` which holds the last time the reward was applicable.
pub fn last_time_reward_applicable() -> U256 {
    math::min(
        U256::from(u64::from(runtime::get_blocktime())),
        get_key::<U256>("staking_rewards_data", "period_finish")
    )
}

/// # Purpose
/// Returns the amount of rewards per token.
/// # Returns
/// * `amount` - an `U256` which holds the amount of rewards per token.
pub fn reward_per_token() -> U256 {
    let total_supply: U256 = get_key("staking_rewards_data", "total_supply");
    let reward_per_token_stored: U256 = get_key("staking_rewards_data", "reward_per_token_stored");
    if (total_supply == U256::from(0)) {
        return reward_per_token_stored;
    }
    let last_update_time: U256 = get_key("staking_rewards_data", "last_update_time");
    let reward_rate: U256 = get_key("staking_rewards_data", "reward_rate");
    return reward_per_token_stored
    .add(
        last_time_reward_applicable()
        .sub(last_update_time)
        .mul(reward_rate)
        .mul(U256::from(10u128.pow(18)))
        .div(total_supply)
    );
}

/// # Purpose
/// Returns the earned reward tokens for a given account.
/// # Arguments
/// * `account` - an `AccountHash` which holds the user's account hash.
/// # Returns
/// * `amount` - an `U256` which holds earned reward tokens amount.
pub fn earned(account: AccountHash) -> U256 {
    get_key::<U256>("balances", &account.to_string())
    .mul(
        reward_per_token()
        .sub(get_key::<U256>("user_reward_per_token_paid", &account.to_string()))
    )
    .div(U256::from(10u128.pow(18)))
    .add(get_key::<U256>("rewards", &account.to_string()))
}

/// # Purpose
/// Withdraws staked tokens.
/// # Arguments
/// * `amount` - a `U256` which holds token's amount that will be withdrawn.
pub fn withdraw(amount: U256) {
    _non_reentrant();
    _update_reward(runtime::get_caller());
    if (amount <= U256::from(0)) {
        _free_entrancy();
        runtime::revert(Error::CannotWithdrawZero);
    }
    set_key(
        "staking_rewards_data",
        "total_supply",
        get_key::<U256>("staking_rewards_data", "total_supply").sub(amount)
    );
    let old_balance: U256 = get_key("balances", &runtime::get_caller().to_string());
    set_key(
        "balances",
        &runtime::get_caller().to_string(),
        old_balance.sub(amount)
    );
    _safe_transfer(
        get_key::<ContractHash>("staking_rewards_data", "staking_token"),
        Key::Account(runtime::get_caller()),
        amount
    );
    _free_entrancy();
}

/// # Purpose
/// Sends reward tokens to the caller if he has any.
pub fn get_reward() {
    _non_reentrant();
    _update_reward(runtime::get_caller());
    let reward: U256 = get_key("rewards", &runtime::get_caller().to_string());
    if (reward > U256::from(0)) {
        set_key(
            "rewards",
            &runtime::get_caller().to_string(),
            U256::from(0)
        );
        _safe_transfer(
            get_key::<ContractHash>("staking_rewards_data", "rewards_token"),
            Key::Account(runtime::get_caller()),
            reward
        );
    }
    _free_entrancy();
}

/* ✖✖✖✖✖✖✖✖✖✖✖ Public functions - End ✖✖✖✖✖✖✖✖✖✖✖ */

#[no_mangle]
pub extern "C" fn call() {
    let owner: AccountHash = runtime::get_named_arg("owner");
    let nominated_owner: AccountHash = AccountHash::new([0u8; 32]);
    let rewards_distribution = ContractHash::from(
        runtime::get_named_arg::<Key>("rewards_distribution").into_hash().unwrap_or_revert()
    );
    let rewards_token = ContractHash::from(
        runtime::get_named_arg::<Key>("rewards_token").into_hash().unwrap_or_revert()
    );
    let staking_token = ContractHash::from(
        runtime::get_named_arg::<Key>("staking_token").into_hash().unwrap_or_revert()
    );

    let dictionary_seed_uref = storage::new_dictionary("staking_rewards_data").unwrap_or_revert();
    storage::dictionary_put(
        dictionary_seed_uref,
        "rewards_token",
        rewards_token,
    );
    storage::dictionary_put(
        dictionary_seed_uref,
        "staking_token",
        staking_token,
    );
    storage::dictionary_put(
        dictionary_seed_uref,
        "period_finish",
        U256::from(0),
    );
    storage::dictionary_put(
        dictionary_seed_uref,
        "reward_rate",
        U256::from(0),
    );
    storage::dictionary_put(
        dictionary_seed_uref,
        "rewards_duration",
        U256::from(604800),
    );
    storage::dictionary_put(
        dictionary_seed_uref,
        "last_update_time",
        U256::from(0),
    );
    storage::dictionary_put(
        dictionary_seed_uref,
        "reward_per_token_stored",
        U256::from(0),
    );
    storage::dictionary_put(
        dictionary_seed_uref,
        "total_supply",
        U256::from(0),
    );
    // from RewardsDistributionRecipient.sol
    storage::dictionary_put(
        dictionary_seed_uref,
        "rewards_distribution",
        rewards_distribution,
    );
    // from Pausable.sol
    storage::dictionary_put(
        dictionary_seed_uref,
        "last_pause_time",
        U256::from(0),
    );
    storage::dictionary_put(
        dictionary_seed_uref,
        "paused",
        false,
    );
    // from Owned.sol
    storage::dictionary_put(
        dictionary_seed_uref,
        "owner",
        owner,
    );
    storage::dictionary_put(
        dictionary_seed_uref,
        "nominated_owner",
        nominated_owner,
    );
    // from ReentrancyGuard.sol
    // _entered and _not_entered are constants, so we don't need to use them
    storage::dictionary_put(
        dictionary_seed_uref,
        "status",
        U256::from(1),
    );
    let rewards_seed_uref = storage::new_dictionary("rewards").unwrap_or_revert();
    storage::dictionary_put(
        rewards_seed_uref,
        &runtime::get_caller().to_string(),
        U256::from(0),
    );
    let user_reward_per_token_paid_seed_uref = storage::new_dictionary("user_reward_per_token_paid").unwrap_or_revert();
    storage::dictionary_put(
        user_reward_per_token_paid_seed_uref,
        &runtime::get_caller().to_string(),
        U256::from(0),
    );
    let balances_seed_uref = storage::new_dictionary("balances").unwrap_or_revert();
    storage::dictionary_put(
        balances_seed_uref,
        &runtime::get_caller().to_string(),
        U256::from(0)
    );

    let mut named_keys = NamedKeys::new();
    named_keys.insert(
        "staking_rewards_data".to_string(), 
        dictionary_seed_uref.into()
    );
    named_keys.insert(
        "rewards".to_string(), 
        rewards_seed_uref.into()
    );
    named_keys.insert(
        "user_reward_per_token_paid".to_string(), 
        user_reward_per_token_paid_seed_uref.into()
    );
    named_keys.insert(
        "balances".to_string(), 
        balances_seed_uref.into()
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
    entry_points.add_entry_point(endpoint(
        "stake",
        vec![Parameter::new("amount", CLType::U256)],
        CLType::Unit,
    ));
    entry_points.add_entry_point(endpoint("exit", vec![], CLType::Unit));
    // from RewardsDistributionRecipient.sol
    entry_points.add_entry_point(endpoint("rewards_distribution", vec![], ContractHash::cl_type()));
    entry_points.add_entry_point(endpoint(
        "set_rewards_distribution", 
        vec![Parameter::new("rewards_distribution", CLType::Key)], 
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
    entry_points.add_entry_point(endpoint(
        "notify_reward_amount", 
        vec![Parameter::new("reward", CLType::U256)], 
        CLType::Unit
    ));
    entry_points.add_entry_point(endpoint(
        "update_period_finish", 
        vec![Parameter::new("timestamp", CLType::U256)], 
        CLType::Unit
    ));
    entry_points.add_entry_point(endpoint(
        "recover_erc20", 
        vec![
            Parameter::new("token_contract_hash", CLType::Key),
            Parameter::new("token_amount", CLType::U256)
            ], 
        CLType::Unit
    ));
    entry_points.add_entry_point(endpoint(
        "set_rewards_duration", 
        vec![Parameter::new("rewards_duration", CLType::U256)], 
        CLType::Unit
    ));

    // let (contract_hash, _) =
    //     storage::new_contract(
    //         entry_points,
    //         Some(named_keys),
    //         Some("staking_rewards".to_string()),
    //         Some("staking_rewards_hash".to_string())
    //     );
    let (contract_package_hash, access_uref) = create_contract_package_at_hash();
    // Add new version to the package.
    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, named_keys);
    // Save contract and contract hash in the caller's context.
    runtime::put_key("StakingRewards", contract_hash.into());
    runtime::put_key("StakingRewards_hash", storage::new_uref(contract_hash).into());
    // Save access_uref
    runtime::put_key("access_uref", access_uref.into());
    // Save contract_hash under the contract's dictionary to be accessed through the contract's endpoints.
    storage::dictionary_put(
        dictionary_seed_uref,
        "contract_hash",
        contract_hash,
    );
}

/* ✖✖✖✖✖✖✖✖✖✖✖ Internal Functions - Start ✖✖✖✖✖✖✖✖✖✖✖ */
fn _only_owner() {
    if (runtime::get_caller() != get_key::<AccountHash>("staking_rewards_data", "owner")) {
        runtime::revert(Error::OnlyOwner);
    }
}

fn _only_nominated_owner() {
    if (runtime::get_caller() != get_key::<AccountHash>("staking_rewards_data", "nominated_owner")) {
        runtime::revert(Error::OnlyNominatedOwner);
    }
}

fn _not_paused() {
    if (get_key::<bool>("staking_rewards_data", "paused")) {
        _free_entrancy();
        runtime::revert(Error::ContractIsPaused);
    }
}

fn _only_rewards_distribution() {
    if (
        get_caller() != 
        Key::Hash(
            get_key::<ContractHash>(
            "staking_rewards_data",
            "rewards_distribution"
            ).value()
        )
    ) {
        runtime::revert(Error::OnlyRewardsDistributionContract);
    }
}

fn _update_reward(account: AccountHash) {
    set_key(
        "staking_rewards_data",
        "reward_per_token_stored",
        reward_per_token()
    );
    set_key(
        "staking_rewards_data",
        "last_update_time",
        last_time_reward_applicable()
    );
    if (account != AccountHash::new([0u8; 32])) {
        set_key(
            "rewards",
            &account.to_string(),
            earned(account)
        );
        set_key(
            "user_reward_per_token_paid",
            &account.to_string(),
            get_key::<U256>("staking_rewards_data", "reward_per_token_stored")
        );
    }
}

fn _non_reentrant() {
    if (get_key::<U256>("staking_rewards_data", "status") == U256::from(2)) {
        runtime::revert(Error::ReentrantCall);
    }
    set_key("staking_rewards_data", "status", U256::from(2));
}

fn _free_entrancy() {
    set_key("staking_rewards_data", "status", U256::from(1));
}

fn _safe_transfer(token: ContractHash, to: Key, value: U256) {
    runtime::call_contract::<()>(
        token,
        "transfer",
        runtime_args! {
            "recipient" => to,
            "amount" => value
        },
    );
}

fn _safe_transfer_from(token: ContractHash, from: Key, to: Key, value: U256) {
    runtime::call_contract::<()>(
        token,
        "transfer_from",
        runtime_args! {
            "owner" => from,
            "recipient" => to,
            "amount" => value
        },
    );
}
/* ✖✖✖✖✖✖✖✖✖✖✖ Internal Functions - End ✖✖✖✖✖✖✖✖✖✖✖ */

fn ret<T: CLTyped + ToBytes>(value: T) {
    runtime::ret(CLValue::from_t(value).unwrap_or_revert())
}

fn get_key<T: FromBytes + CLTyped + Default>(dictionary_name: &str, key: &str) -> T {
    let dictionary_seed_uref = get_dictionary_seed_uref(dictionary_name);
    storage::dictionary_get(dictionary_seed_uref, key).unwrap_or_default().unwrap_or_default()
}

fn set_key<T: ToBytes + CLTyped>(dictionary_name: &str, key: &str, value: T) { 
    let dictionary_seed_uref = get_dictionary_seed_uref(dictionary_name);
    storage::dictionary_put(dictionary_seed_uref, key, value)
}

fn get_dictionary_seed_uref(name: &str) -> URef {
    let dictionary_seed_uref = match runtime::get_key(name) {
        Some(key) => key.into_uref().unwrap_or_revert(),
        None => {
            let new_dict = storage::new_dictionary(name).unwrap_or_revert();
            let key = storage::new_uref(new_dict).into();
            runtime::put_key(name, key);
            new_dict
        },
    };
    dictionary_seed_uref
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

fn get_caller() -> Key {
    let mut callstack = runtime::get_call_stack();
    callstack.pop();
    match callstack.last().unwrap_or_revert() {
        CallStackElement::Session { account_hash } => (*account_hash).into(),
        CallStackElement::StoredSession {
            account_hash,
            contract_package_hash: _,
            contract_hash: _,
        } => (*account_hash).into(),
        CallStackElement::StoredContract {
            contract_package_hash: _,
            contract_hash,
        } => (*contract_hash).into(),
    }
}