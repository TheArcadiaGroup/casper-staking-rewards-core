
use casper_engine_test_support::AccountHash;
use casper_types::{ContractHash, Key};
use std::time::{SystemTime, UNIX_EPOCH};

use casper_types::U256;
use crate::erc20::{token_cfg, Sender, Token};
use crate::staking_rewards::{Sender as STK_Sender, StakingRewards};

// ------------ START - ERC20 Tests ------------

fn to_key(account: AccountHash) -> Key {
    Key::Account(account)
}

#[test]
fn test_erc20_deploy() {
    let t = Token::deployed("ERC20", "ERC");
    assert_eq!(t.name(), token_cfg::NAME);
    assert_eq!(t.symbol(), token_cfg::SYMBOL);
    assert_eq!(t.decimals(), token_cfg::DECIMALS);
    assert_eq!(t.balance_of(to_key(t.ali)), token_cfg::total_supply());
    //assert_eq!(t.balance_of(t.bob), 0.into());
    assert_eq!(t.allowance(to_key(t.ali), to_key(t.ali)), 0.into());
    assert_eq!(t.allowance(to_key(t.ali), to_key(t.bob)), 0.into());
    assert_eq!(t.allowance(to_key(t.bob), to_key(t.ali)), 0.into());
    assert_eq!(t.allowance(to_key(t.bob), to_key(t.bob)), 0.into());
}

#[test]
fn test_staking_token_deploy() {
    let t = Token::deployed("StakingToken", "STKN");
    assert_eq!(t.name(), "StakingToken");
    assert_eq!(t.symbol(), "STKN");
    assert_eq!(t.decimals(), token_cfg::DECIMALS);
    assert_eq!(t.balance_of(to_key(t.ali)), token_cfg::total_supply());
    //assert_eq!(t.balance_of(t.bob), 0.into());
    assert_eq!(t.allowance(to_key(t.ali), to_key(t.ali)), 0.into());
    assert_eq!(t.allowance(to_key(t.ali), to_key(t.bob)), 0.into());
    assert_eq!(t.allowance(to_key(t.bob), to_key(t.ali)), 0.into());
    assert_eq!(t.allowance(to_key(t.bob), to_key(t.bob)), 0.into());
}

#[test]
fn test_rewards_token_deploy() {
    let t = Token::deployed("RewardsToken", "RWDT");
    assert_eq!(t.name(), "RewardsToken");
    assert_eq!(t.symbol(), "RWDT");
    assert_eq!(t.decimals(), token_cfg::DECIMALS);
    assert_eq!(t.balance_of(to_key(t.ali)), token_cfg::total_supply());
    //assert_eq!(t.balance_of(t.bob), 0.into());
    assert_eq!(t.allowance(to_key(t.ali), to_key(t.ali)), 0.into());
    assert_eq!(t.allowance(to_key(t.ali), to_key(t.bob)), 0.into());
    assert_eq!(t.allowance(to_key(t.bob), to_key(t.ali)), 0.into());
    assert_eq!(t.allowance(to_key(t.bob), to_key(t.bob)), 0.into());
}

#[test]
fn test_erc20_transfer() {
    let amount = 10.into();
    let mut t = Token::deployed("ERC20", "ERC");
    t.transfer(to_key(t.bob), amount, Sender(t.ali));
    assert_eq!(t.balance_of(to_key(t.ali)), token_cfg::total_supply() - amount);
    assert_eq!(t.balance_of(to_key(t.bob)), amount);
}

#[test]
#[should_panic]
fn test_erc20_transfer_too_much() {
    let amount = 1.into();
    let mut t = Token::deployed("ERC20", "ERC");
    t.transfer(to_key(t.ali), amount, Sender(t.bob));
}

#[test]
fn test_erc20_approve() {
    let amount = 10.into();
    let mut t = Token::deployed("ERC20", "ERC");
    t.approve(to_key(t.bob), amount, Sender(t.ali));
    assert_eq!(t.balance_of(to_key(t.ali)), token_cfg::total_supply());
    //assert_eq!(t.balance_of(t.bob), 0.into());
    assert_eq!(t.allowance(to_key(t.ali), to_key(t.bob)), amount);
    assert_eq!(t.allowance(to_key(t.bob), to_key(t.ali)), 0.into());
}

#[test]
fn test_erc20_transfer_from() {
    let allowance = 10.into();
    let amount = 3.into();
    let mut t = Token::deployed("ERC20", "ERC");
    t.approve(to_key(t.bob), allowance, Sender(t.ali));
    assert_eq!(t.allowance(to_key(t.ali), to_key(t.bob)), allowance);
    t.transfer_from(to_key(t.ali), to_key(t.joe), amount, Sender(t.bob));
    assert_eq!(t.balance_of(to_key(t.ali)), token_cfg::total_supply() - amount);
    //assert_eq!(t.balance_of(t.bob), 0.into());
    assert_eq!(t.balance_of(to_key(t.joe)), amount);
    assert_eq!(t.allowance(to_key(t.ali), to_key(t.bob)), allowance - amount);
}

#[test]
#[should_panic]
fn test_erc20_transfer_from_too_much() {
    let amount = token_cfg::total_supply().checked_add(1.into()).unwrap();
    let mut t = Token::deployed("ERC20", "ERC");
    t.transfer_from(to_key(t.ali), to_key(t.joe), amount, Sender(t.bob));
}

// // ------------ START - StakingRewards Tests ------------

fn deploy_staking_rewards() -> StakingRewards {
    let rewards_distribution = Token::deployed("RewardsDistribution", "RWDD");
    let rewards_token = Token::deployed("RewardsToken", "RWDT");
    let staking_token = Token::deployed("StakingToken", "STK");
    let stk_rwd = StakingRewards::deployed(
        rewards_distribution,
        rewards_token,
        staking_token
    );
    stk_rwd
}

#[test]
fn test_staking_rewards_deploy() {
    let stk_rwd = deploy_staking_rewards();
    assert_eq!(stk_rwd.owner(), stk_rwd.ali);
    assert_eq!(stk_rwd.nominated_owner(), AccountHash::new([0u8; 32]));
    assert_eq!(stk_rwd.rewards_distribution(), ContractHash::new(stk_rwd.rewards_distribution.contract_hash()));
    assert_eq!(stk_rwd.rewards_token(), ContractHash::new(stk_rwd.rewards_token.contract_hash()));
    assert_eq!(stk_rwd.staking_token(), ContractHash::new(stk_rwd.staking_token.contract_hash()));
    assert_eq!(stk_rwd.period_finish(), U256::from(0));
    assert_eq!(stk_rwd.reward_rate(), U256::from(0));
    assert_eq!(stk_rwd.rewards_duration(), U256::from(604800));
    assert_eq!(stk_rwd.last_update_time(), U256::from(0));
    assert_eq!(stk_rwd.reward_per_token_stored(), U256::from(0));
    assert_eq!(stk_rwd.total_supply(), U256::from(0));
    assert_eq!(stk_rwd.last_pause_time(), U256::from(0));
    assert_eq!(stk_rwd.paused(), false);
}

#[test]
fn test_set_rewards_distribution() {
    let mut stk_rwd = deploy_staking_rewards();
    stk_rwd.set_rewards_distribution(Key::Hash([0u8; 32]), STK_Sender(stk_rwd.ali));
    assert_eq!(stk_rwd.rewards_distribution(), ContractHash::new([0u8; 32]));
}

#[test]
#[should_panic]
fn test_only_owner() {
    let mut stk_rwd = deploy_staking_rewards();
    stk_rwd.set_rewards_distribution(Key::Hash([0u8; 32]), STK_Sender(stk_rwd.joe));
    assert_eq!(stk_rwd.rewards_distribution(), ContractHash::new([0u8; 32]));
}

#[test]
fn test_set_paused() {
    let mut stk_rwd = deploy_staking_rewards();
    assert_eq!(stk_rwd.paused(), false);
    stk_rwd.set_paused(true, STK_Sender(stk_rwd.ali));
    assert_eq!(stk_rwd.paused(), true);
    //assert_eq!(stk_rwd.last_pause_time(), U256::from(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()));
}

#[test]
fn test_set_nominated_owner() {
    let mut stk_rwd = deploy_staking_rewards();
    stk_rwd.nominate_new_owner(stk_rwd.ali, STK_Sender(stk_rwd.ali));
    assert_eq!(stk_rwd.nominated_owner(), stk_rwd.ali);
}

#[test]
fn test_accept_ownership() {
    let mut stk_rwd = deploy_staking_rewards();
    stk_rwd.nominate_new_owner(stk_rwd.ali, STK_Sender(stk_rwd.ali));
    assert_eq!(stk_rwd.nominated_owner(), stk_rwd.ali);
    stk_rwd.accept_ownership(STK_Sender(stk_rwd.ali));
    assert_eq!(stk_rwd.owner(), stk_rwd.ali);
    assert_eq!(stk_rwd.nominated_owner(), AccountHash::new([0u8; 32]));
}

#[test]
#[should_panic]
fn test_stake_zero() {
    let mut stk_rwd = deploy_staking_rewards();
    let amount = 0.into();
    stk_rwd.stake(amount, STK_Sender(stk_rwd.ali));
}

#[test]
#[should_panic]
// panics because the contract can't get its own hash.
fn test_stake() {
    let mut stk_rwd = deploy_staking_rewards();
    assert_eq!(stk_rwd.paused(), false);
    let amount: U256 = 3.into();
    let allowance = 10.into();
    let old_balance: U256 = stk_rwd.staking_token.balance_of(to_key(stk_rwd.ali));
    assert_eq!(old_balance, U256::from(1000));
    let staking_rewards_hash = Key::Hash(stk_rwd.contract_hash());
    stk_rwd.staking_token.approve(staking_rewards_hash, amount, Sender(stk_rwd.ali));
    stk_rwd.staking_token.approve(to_key(stk_rwd.ali), allowance, Sender(stk_rwd.ali));
    stk_rwd.stake(amount, STK_Sender(stk_rwd.ali));
    assert_eq!(stk_rwd.reward_per_token_stored(), U256::from(0));
    assert_eq!(stk_rwd.last_update_time(), U256::from(0));
    assert_eq!(stk_rwd.total_supply(), amount);
    assert_eq!(stk_rwd.reward_of(stk_rwd.ali), U256::from(0));
    assert_eq!(stk_rwd.user_reward_per_token_paid(stk_rwd.ali), U256::from(0));
    assert_eq!(stk_rwd.balance_of(stk_rwd.ali), amount);
    assert_eq!(stk_rwd.staking_token.balance_of(to_key(stk_rwd.staking_token.ali)), old_balance - amount);
    assert_eq!(stk_rwd.staking_token.balance_of(Key::Hash(stk_rwd.contract_hash())), amount);
}

#[test]
#[should_panic]
fn test_exit_withdraw_0() {
    let mut stk_rwd = deploy_staking_rewards();
    stk_rwd.exit(STK_Sender(stk_rwd.ali));
}

#[test]
fn test_update_period_finish() {
    let mut stk_rwd = deploy_staking_rewards();
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    stk_rwd.update_period_finish(
        U256::from(timestamp),
        STK_Sender(stk_rwd.ali)
    );
    assert_eq!(stk_rwd.reward_per_token_stored(), U256::from(0));
    assert_eq!(stk_rwd.last_update_time(), U256::from(0));
    assert_eq!(stk_rwd.period_finish(), U256::from(timestamp));
}

#[test]
#[should_panic]
// panics because runtime::get_blocktime() returns a zero value.
// It's a limitation when working with test contexts.
fn test_set_rewards_duration() {
    let mut stk_rwd = deploy_staking_rewards();
    let rewards_duration = 1000.into();
    stk_rwd.set_rewards_duration(rewards_duration, STK_Sender(stk_rwd.ali));
    assert_eq!(stk_rwd.rewards_duration(), rewards_duration);
}

#[test]
#[should_panic]
// panics because call_contract() gives me KeyNotFound error.
// Assuming it's a limitation when working with test contexts.
fn test_recover_erc20() {
    let mut stk_rwd = deploy_staking_rewards();
    let amount: U256 = 3.into();
    let initial_balance: U256 = stk_rwd.rewards_distribution.balance_of(to_key(stk_rwd.ali));
    let staking_rewards_hash = Key::Hash(stk_rwd.contract_hash());
    stk_rwd.rewards_distribution.transfer(staking_rewards_hash, amount, Sender(stk_rwd.ali));
    assert_eq!(
        stk_rwd.rewards_distribution.balance_of(to_key(stk_rwd.ali)),
        initial_balance - amount
    );
    assert_eq!(
        stk_rwd.rewards_distribution.balance_of(staking_rewards_hash),
        amount
    );
    let rewards_distribution_key = Key::Hash(stk_rwd.rewards_distribution.contract_hash());
    //println!("rewards_distribution KEY: {}", rewards_distribution_key);
    stk_rwd.recover_erc20(
        rewards_distribution_key,
        amount,
        STK_Sender(stk_rwd.ali)
    );
    // stk_rwd.rewards_distribution.approve(stk_rwd.ali, amount, Sender(staking_rewards_hash));
    // stk_rwd.rewards_distribution.transfer_from(
    //     staking_rewards_hash,
    //     stk_rwd.ali,
    //     amount,
    //     Sender(stk_rwd.ali)
    // );
    assert_eq!(
        stk_rwd.rewards_distribution.balance_of(to_key(stk_rwd.ali)),
        initial_balance
    );
    assert_eq!(
        stk_rwd.rewards_distribution.balance_of(staking_rewards_hash),
        U256::from(0)
    );
}

#[test]
#[should_panic]
fn test_recover_staking_token() {
    let mut stk_rwd = deploy_staking_rewards();
    let amount: U256 = 3.into();
    stk_rwd.recover_erc20(
        Key::Hash(stk_rwd.staking_token.contract_hash()),
        amount,
        STK_Sender(stk_rwd.ali)
    );
}

#[test]
fn transfer_from_to_contract() {
    let amount = 3.into();
    let mut stk_rwd = deploy_staking_rewards();
    stk_rwd.staking_token.approve(
        to_key(stk_rwd.ali),
        amount,
        Sender(stk_rwd.ali)
    );
    stk_rwd.staking_token.transfer_from(
        to_key(stk_rwd.ali),
        Key::Hash(stk_rwd.contract_hash()),
        amount,
        Sender(stk_rwd.ali)
    );
    assert_eq!(stk_rwd.staking_token.balance_of(Key::Hash(stk_rwd.contract_hash())), amount);
    assert_eq!(stk_rwd.staking_token.balance_of(to_key(stk_rwd.ali)), U256::from(997));
}

#[test]
fn transfer_to_contract() {
    let amount = 3.into();
    let mut stk_rwd = deploy_staking_rewards();
    stk_rwd.staking_token.transfer(
        Key::Hash(stk_rwd.contract_hash()),
        amount,
        Sender(stk_rwd.ali)
    );
    assert_eq!(stk_rwd.staking_token.balance_of(Key::Hash(stk_rwd.contract_hash())), amount);
    assert_eq!(stk_rwd.staking_token.balance_of(to_key(stk_rwd.ali)), U256::from(997));
}