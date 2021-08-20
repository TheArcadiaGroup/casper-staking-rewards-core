//use casper_types::U256;
use crate::erc20::{token_cfg, Sender, Token};

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
    let t = Token::deployed("STAKING_TOKEN", "STKN");
    assert_eq!(t.name(), "STAKING_TOKEN");
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
    let t = Token::deployed("REWARDS_TOKEN", "RWDN");
    assert_eq!(t.name(), "REWARDS_TOKEN");
    assert_eq!(t.symbol(), "RWDN");
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