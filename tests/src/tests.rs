use casper_engine_test_support::AccountHash;
use casper_types::ContractHash;
//use std::time::{SystemTime, UNIX_EPOCH};

use casper_types::U256;
use crate::erc20::{token_cfg, Sender, Token};
use crate::staking_rewards::{Sender as STK_Sender, StakingRewards};

// ------------ START - ERC20 Tests ------------

#[test]
fn test_erc20_deploy() {
    let t = Token::deployed("ERC20", "ERC");
    assert_eq!(t.name(), token_cfg::NAME);
    assert_eq!(t.symbol(), token_cfg::SYMBOL);
    assert_eq!(t.decimals(), token_cfg::DECIMALS);
    assert_eq!(t.balance_of(t.ali), token_cfg::total_supply());
    //assert_eq!(t.balance_of(t.bob), 0.into());
    assert_eq!(t.allowance(t.ali, t.ali), 0.into());
    assert_eq!(t.allowance(t.ali, t.bob), 0.into());
    assert_eq!(t.allowance(t.bob, t.ali), 0.into());
    assert_eq!(t.allowance(t.bob, t.bob), 0.into());
}

#[test]
fn test_staking_token_deploy() {
    let t = Token::deployed("StakingToken", "STKN");
    assert_eq!(t.name(), "StakingToken");
    assert_eq!(t.symbol(), "STKN");
    assert_eq!(t.decimals(), token_cfg::DECIMALS);
    assert_eq!(t.balance_of(t.ali), token_cfg::total_supply());
    //assert_eq!(t.balance_of(t.bob), 0.into());
    assert_eq!(t.allowance(t.ali, t.ali), 0.into());
    assert_eq!(t.allowance(t.ali, t.bob), 0.into());
    assert_eq!(t.allowance(t.bob, t.ali), 0.into());
    assert_eq!(t.allowance(t.bob, t.bob), 0.into());
}

#[test]
fn test_rewards_token_deploy() {
    let t = Token::deployed("RewardsToken", "RWDT");
    assert_eq!(t.name(), "RewardsToken");
    assert_eq!(t.symbol(), "RWDT");
    assert_eq!(t.decimals(), token_cfg::DECIMALS);
    assert_eq!(t.balance_of(t.ali), token_cfg::total_supply());
    //assert_eq!(t.balance_of(t.bob), 0.into());
    assert_eq!(t.allowance(t.ali, t.ali), 0.into());
    assert_eq!(t.allowance(t.ali, t.bob), 0.into());
    assert_eq!(t.allowance(t.bob, t.ali), 0.into());
    assert_eq!(t.allowance(t.bob, t.bob), 0.into());
}

#[test]
fn test_erc20_transfer() {
    let amount = 10.into();
    let mut t = Token::deployed("ERC20", "ERC");
    t.transfer(t.bob, amount, Sender(t.ali));
    assert_eq!(t.balance_of(t.ali), token_cfg::total_supply() - amount);
    assert_eq!(t.balance_of(t.bob), amount);
}

#[test]
#[should_panic]
fn test_erc20_transfer_too_much() {
    let amount = 1.into();
    let mut t = Token::deployed("ERC20", "ERC");
    t.transfer(t.ali, amount, Sender(t.bob));
}

#[test]
fn test_erc20_approve() {
    let amount = 10.into();
    let mut t = Token::deployed("ERC20", "ERC");
    t.approve(t.bob, amount, Sender(t.ali));
    assert_eq!(t.balance_of(t.ali), token_cfg::total_supply());
    //assert_eq!(t.balance_of(t.bob), 0.into());
    assert_eq!(t.allowance(t.ali, t.bob), amount);
    assert_eq!(t.allowance(t.bob, t.ali), 0.into());
}

#[test]
fn test_erc20_transfer_from() {
    let allowance = 10.into();
    let amount = 3.into();
    let mut t = Token::deployed("ERC20", "ERC");
    t.approve(t.bob, allowance, Sender(t.ali));
    t.transfer_from(t.ali, t.joe, amount, Sender(t.bob));
    assert_eq!(t.balance_of(t.ali), token_cfg::total_supply() - amount);
    //assert_eq!(t.balance_of(t.bob), 0.into());
    assert_eq!(t.balance_of(t.joe), amount);
    assert_eq!(t.allowance(t.ali, t.bob), allowance - amount);
}

#[test]
#[should_panic]
fn test_erc20_transfer_from_too_much() {
    let amount = token_cfg::total_supply().checked_add(1.into()).unwrap();
    let mut t = Token::deployed("ERC20", "ERC");
    t.transfer_from(t.ali, t.joe, amount, Sender(t.bob));
}

// ------------ START - StakingRewards Tests ------------

fn deploy_staking_rewards() -> StakingRewards {
    let rewards_distribution = Token::deployed("RewardsDistribution", "RWDD");
    let rewards_token = Token::deployed("RewardsToken", "RWDT");
    let staking_token = Token::deployed("StakingToken", "STK");
    let stk_rwd = StakingRewards::deployed(
        ContractHash::new(rewards_distribution.contract_hash()),
        ContractHash::new(rewards_token.contract_hash()),
        ContractHash::new(staking_token.contract_hash())
    );
    stk_rwd
}

#[test]
fn test_staking_rewards_deploy() {
    let rewards_distribution = Token::deployed("RewardsDistribution", "RWDD");
    let rewards_token = Token::deployed("RewardsToken", "RWDT");
    let staking_token = Token::deployed("StakingToken", "STK");
    let stk_rwd = StakingRewards::deployed(
        ContractHash::new(rewards_distribution.contract_hash()),
        ContractHash::new(rewards_token.contract_hash()),
        ContractHash::new(staking_token.contract_hash())
    );
    assert_eq!(stk_rwd.owner(), stk_rwd.ali);
    assert_eq!(stk_rwd.nominated_owner(), AccountHash::new([0u8; 32]));
    assert_eq!(stk_rwd.rewards_distribution(), ContractHash::new(rewards_distribution.contract_hash()));
    assert_eq!(stk_rwd.rewards_token(), ContractHash::new(rewards_token.contract_hash()));
    assert_eq!(stk_rwd.staking_token(), ContractHash::new(staking_token.contract_hash()));
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
    stk_rwd.set_rewards_distribution(ContractHash::new([0u8; 32]), STK_Sender(stk_rwd.ali));
    assert_eq!(stk_rwd.rewards_distribution(), ContractHash::new([0u8; 32]));
}

#[test]
#[should_panic]
fn test_only_owner() {
    let mut stk_rwd = deploy_staking_rewards();
    stk_rwd.set_rewards_distribution(ContractHash::new([0u8; 32]), STK_Sender(stk_rwd.joe));
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