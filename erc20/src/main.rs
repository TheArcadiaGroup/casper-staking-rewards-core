#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(non_snake_case)]

extern crate alloc;

use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::String,
};
use core::convert::TryInto;

use contract::{contract_api::{runtime::{self, blake2b}, storage}, unwrap_or_revert::UnwrapOrRevert};
use types::{ApiError, CLType, CLTyped, CLValue, ContractHash, Group, Key, Parameter, RuntimeArgs, U256, URef, account::AccountHash, bytesrepr::{FromBytes, ToBytes}, contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys}, runtime_args, system::CallStackElement};

#[no_mangle]
pub extern "C" fn name() {
    let val: String = get_key("token_metadata", "name");
    ret(val)
}

#[no_mangle]
pub extern "C" fn symbol() {
    let val: String = get_key("token_metadata", "symbol");
    ret(val)
}

#[no_mangle]
pub extern "C" fn decimals() {
    let val: u8 = get_key("token_metadata", "decimals");
    ret(val)
}

#[no_mangle]
pub extern "C" fn total_supply() {
    let val: U256 = get_key("token_metadata", "total_supply");
    ret(val)
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let account: Key = runtime::get_named_arg("account");
    let val: U256 = get_key("balances", &key_to_str(&account));
    ret(val)
}

#[no_mangle]
pub extern "C" fn allowance() {
    let owner: Key = runtime::get_named_arg("owner");
    let spender: Key = runtime::get_named_arg("spender");
    let val: U256 = get_key_runtime(&allowance_key(&owner, &spender));
    ret(val)
}

#[no_mangle]
pub extern "C" fn approve() {
    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    _approve(
        Key::Account(runtime::get_caller()),
        spender,
        amount
    );
}

#[no_mangle]
pub extern "C" fn transfer() {
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    _transfer(
        get_caller(),
        recipient,
        amount
    );
}

#[no_mangle]
pub extern "C" fn transfer_from() {
    let owner: Key = runtime::get_named_arg("owner");
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    _transfer_from(owner, recipient, amount);
}

// #[no_mangle]
// pub extern "C" fn call_notify_reward_amount() {
//     runtime::call_contract::<()>(
//         get_key::<ContractHash>("token_metadata", "staking_rewards_hash"),
//         "notify_reward_amount",
//         runtime_args! {
//             "reward" => runtime::get_named_arg::<U256>("reward"),
//         },
//     );
// }

#[no_mangle]
pub extern "C" fn call() {
    let token_name: String = runtime::get_named_arg("token_name");
    let token_symbol: String = runtime::get_named_arg("token_symbol");
    let token_decimals: u8 = runtime::get_named_arg("token_decimals");
    let token_total_supply: U256 = runtime::get_named_arg("token_total_supply");
    // let staking_rewards_hash: ContractHash = ContractHash::from(
    //     runtime::get_named_arg::<Key>("staking_rewards_key").into_hash().unwrap_or_revert()
    // );

    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(endpoint("name", vec![], CLType::String));
    entry_points.add_entry_point(endpoint("symbol", vec![], CLType::String));
    entry_points.add_entry_point(endpoint("decimals", vec![], CLType::U8));
    entry_points.add_entry_point(endpoint("total_supply", vec![], CLType::U32));
    entry_points.add_entry_point(endpoint(
        "transfer",
        vec![
            Parameter::new("recipient", CLType::Key),
            Parameter::new("amount", CLType::U256),
        ],
        CLType::Unit,
    ));
    entry_points.add_entry_point(endpoint(
        "balance_of",
        vec![Parameter::new("account", CLType::Key)],
        CLType::U256,
    ));
    entry_points.add_entry_point(endpoint(
        "allowance",
        vec![
            Parameter::new("owner", CLType::Key),
            Parameter::new("spender", CLType::Key),
        ],
        CLType::U256,
    ));
    entry_points.add_entry_point(endpoint(
        "approve",
        vec![
            Parameter::new("spender", CLType::Key),
            Parameter::new("amount", CLType::U256),
        ],
        CLType::Unit,
    ));
    entry_points.add_entry_point(endpoint(
        "transfer_from",
        vec![
            Parameter::new("owner", CLType::Key),
            Parameter::new("recipient", CLType::Key),
            Parameter::new("amount", CLType::U256),
        ],
        CLType::Unit,
    ));
    // entry_points.add_entry_point(endpoint(
    //     "call_notify_reward_amount",
    //     vec![
    //         Parameter::new("reward", CLType::U256),
    //     ],
    //     CLType::Unit,
    // ));

    let dictionary_seed_uref = storage::new_dictionary("token_metadata").unwrap_or_revert();
    storage::dictionary_put(
        dictionary_seed_uref,
        "name",
        token_name.clone()
    );
    storage::dictionary_put(
        dictionary_seed_uref,
        "symbol",
        token_symbol
    );
    storage::dictionary_put(
        dictionary_seed_uref,
        "decimals",
        token_decimals
    );
    storage::dictionary_put(
        dictionary_seed_uref,
        "total_supply",
        token_total_supply
    );
    // storage::dictionary_put(
    //     dictionary_seed_uref,
    //     "staking_rewards_hash",
    //     staking_rewards_hash
    // );
    let balances_seed_uref = storage::new_dictionary("balances").unwrap_or_revert();
    storage::dictionary_put(
        balances_seed_uref,
        &key_to_str(&Key::Account(runtime::get_caller())),
        token_total_supply
    );
    let mut named_keys = NamedKeys::new();
    named_keys.insert(
        "token_metadata".to_string(), 
        dictionary_seed_uref.into()
    );
    named_keys.insert(
        "balances".to_string(), 
        balances_seed_uref.into()
    );

    let (contract_hash, _) =
        storage::new_locked_contract(entry_points, Some(named_keys), None, None);
    runtime::put_key(&token_name, contract_hash.into());
    runtime::put_key([&token_name, "_hash"].join("").as_str(), storage::new_uref(contract_hash).into());
}

fn _transfer(sender: Key, recipient: Key, amount: U256) {
    let new_sender_balance: U256 = (get_key::<U256>("balances", &key_to_str(&sender)) - amount);
    set_key("balances", &key_to_str(&sender), new_sender_balance);
    let new_recipient_balance: U256 = (get_key::<U256>("balances", &key_to_str(&recipient)) + amount);
    set_key("balances", &key_to_str(&recipient), new_recipient_balance);
}

fn _transfer_from(owner: Key, recipient: Key, amount: U256) {
    let key = allowance_key(&owner, &Key::Account(runtime::get_caller()));
    _transfer(owner, recipient, amount);
    _approve(
        owner,
        Key::Account(runtime::get_caller()),
        (get_key_runtime::<U256>(&key) - amount),
    );
}

fn _approve(owner: Key, spender: Key, amount: U256) {
    set_key_runtime(&allowance_key(&owner, &spender), amount);
}

fn ret<T: CLTyped + ToBytes>(value: T) {
    runtime::ret(CLValue::from_t(value).unwrap_or_revert())
}

fn key_to_str(key: &Key) -> String {
    match key {
        Key::Account(account) => account.to_string(),
        Key::Hash(package) => hex::encode(package),
        _ => runtime::revert(ApiError::UnexpectedKeyVariant),
    }
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

fn get_key_runtime<T: FromBytes + CLTyped + Default>(name: &str) -> T {
    match runtime::get_key(name) {
        None => Default::default(),
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            storage::read(key).unwrap_or_revert().unwrap_or_revert()
        }
    }
}

fn set_key_runtime<T: ToBytes + CLTyped>(name: &str, value: T) {
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

fn get_key<T: FromBytes + CLTyped + Default>(dictionary_name: &str, key: &str) -> T {
    let dictionary_seed_uref = get_dictionary_seed_uref(dictionary_name);
    storage::dictionary_get(dictionary_seed_uref, key).unwrap_or_default().unwrap_or_default()
}

fn set_key<T: ToBytes + CLTyped>(dictionary_name: &str, key: &str, value: T) { 
    let dictionary_seed_uref = get_dictionary_seed_uref(dictionary_name);
    storage::dictionary_put(dictionary_seed_uref, key, value)
}

fn allowance_key(owner: &Key, sender: &Key) -> String {
    format!("allowances_{}_{}", owner, sender)
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

fn main() {
    println!("Hello, ERC20!");
}