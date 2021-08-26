use casper_engine_test_support::{Code, Hash, SessionBuilder, TestContext, TestContextBuilder};
use casper_types::{AsymmetricType, CLTyped, ContractHash, PublicKey, RuntimeArgs, U256, U512, account::AccountHash, bytesrepr::FromBytes, runtime_args};
use crate::erc20::Token;

// contains methods that can simulate a real-world deployment (storing the contract in the blockchain)
// and transactions to invoke the methods in the contract.

pub struct Sender(pub AccountHash);

pub struct StakingRewards {
    context: TestContext,
    pub ali: AccountHash,
    pub bob: AccountHash,
    pub joe: AccountHash,
    pub rewards_distribution: Token,
    pub rewards_token: Token,
    pub staking_token: Token,
}

impl StakingRewards {
    pub fn deployed(
        rewards_distribution: Token,
        rewards_token: Token,
        staking_token: Token
    ) -> StakingRewards {
        let ali = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
        let bob = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();
        let joe = PublicKey::ed25519_from_bytes([9u8; 32]).unwrap();
        
        let mut context = TestContextBuilder::new()
            .with_public_key(ali.clone(), U512::from(500_000_000_000_000_000u64))
            .with_public_key(bob.clone(), U512::from(500_000_000_000_000_000u64))
            .build();
        let session_code = Code::from("staking_rewards.wasm");
        let session_args = runtime_args! {
            "owner" => ali.to_account_hash(),
            "rewards_distribution" => ContractHash::new(rewards_distribution.contract_hash()).to_formatted_string(),
            "rewards_token" => ContractHash::new(rewards_token.contract_hash()).to_formatted_string(),
            "staking_token" => ContractHash::new(staking_token.contract_hash()).to_formatted_string()
        };
        let session = SessionBuilder::new(session_code, session_args)
            .with_address((&ali).to_account_hash())
            .with_authorization_keys(&[ali.to_account_hash()])
            .build();
        context.run(session);
        StakingRewards {
            context,
            ali: ali.to_account_hash(),
            bob: bob.to_account_hash(),
            joe: joe.to_account_hash(),
            rewards_distribution: rewards_distribution,
            rewards_token: rewards_token,
            staking_token: staking_token
        }
    }

    pub fn contract_hash(&self) -> Hash {
        self.context
            .query(self.ali, &[format!("{}_hash", "StakingRewards")])
            .unwrap_or_else(|_| panic!("{} contract not found", "StakingRewards"))
            .into_t()
            .unwrap_or_else(|_| panic!("{} has wrong type", "StakingRewards"))
    }

    /// query a contract's dictionary's key.
    fn query_contract_dictionary<T: CLTyped + FromBytes>(
        &self,
        key: AccountHash,
        context: &TestContext,
        dictionary_name: String,
        name: String,
    ) -> Option<T> {
        match context.query_dictionary_item(key.into(), Some(dictionary_name), name.clone()) {
            Err(_) => None,
            Ok(maybe_value) => {
                let value = maybe_value
                    .into_t()
                    .unwrap_or_else(|_| panic!("{} is not the expected type.", name));
                Some(value)
            }
        }
    }

    /// call a contract's specific entry point.
    fn call(&mut self, sender: Sender, method: &str, args: RuntimeArgs) {
        let Sender(address) = sender;
        let code = Code::Hash(self.contract_hash(), method.to_string());
        let session = SessionBuilder::new(code, args)
            .with_address(address)
            .with_authorization_keys(&[address])
            .build();
        self.context.run(session);
    }
    /* ✖✖✖✖✖✖✖✖✖✖✖ Public getters - Start ✖✖✖✖✖✖✖✖✖✖✖ */
    pub fn rewards_token(&self) -> ContractHash {
        self.query_contract_dictionary(
            self.ali,
            &self.context,
            "staking_rewards_data".to_string(),
            "rewards_token".to_string()
        ).unwrap()
    }

    pub fn staking_token(&self) -> ContractHash {
        self.query_contract_dictionary(
            self.ali,
            &self.context,
            "staking_rewards_data".to_string(),
            "staking_token".to_string()
        ).unwrap()
    }

    pub fn period_finish(&self) -> U256 {
        self.query_contract_dictionary(
            self.ali,
            &self.context,
            "staking_rewards_data".to_string(),
            "period_finish".to_string()
        ).unwrap()
    }

    pub fn reward_rate(&self) -> U256 {
        self.query_contract_dictionary(
            self.ali,
            &self.context,
            "staking_rewards_data".to_string(),
            "reward_rate".to_string()
        ).unwrap()
    }

    pub fn rewards_duration(&self) -> U256 {
        self.query_contract_dictionary(
            self.ali,
            &self.context,
            "staking_rewards_data".to_string(),
            "rewards_duration".to_string()
        ).unwrap()
    }

    pub fn last_update_time(&self) -> U256 {
        self.query_contract_dictionary(
            self.ali,
            &self.context,
            "staking_rewards_data".to_string(),
            "last_update_time".to_string()
        ).unwrap()
    }

    pub fn reward_per_token_stored(&self) -> U256 {
        self.query_contract_dictionary(
            self.ali,
            &self.context,
            "staking_rewards_data".to_string(),
            "reward_per_token_stored".to_string()
        ).unwrap()
    }

    pub fn user_reward_per_token_paid(&self, owner: AccountHash) -> U256 {
        self.query_contract_dictionary(
            self.ali,
            &self.context,
            "user_reward_per_token_paid".to_string(),
            owner.to_string()
        ).unwrap()
    }

    pub fn reward_of(&self, owner: AccountHash) -> U256 {
        self.query_contract_dictionary(
            self.ali,
            &self.context,
            "rewards".to_string(),
            owner.to_string()
        ).unwrap()
    }

    pub fn total_supply(&self) -> U256 {
        self.query_contract_dictionary(
            self.ali,
            &self.context,
            "staking_rewards_data".to_string(),
            "total_supply".to_string()
        ).unwrap()
    }

    pub fn balance_of(&self, owner: AccountHash) -> U256 {
        self.query_contract_dictionary(
            self.ali,
            &self.context,
            "balances".to_string(),
            owner.to_string()
        ).unwrap()
    }

    pub fn rewards_distribution(&self) -> ContractHash {
        self.query_contract_dictionary(
            self.ali,
            &self.context,
            "staking_rewards_data".to_string(),
            "rewards_distribution".to_string()
        ).unwrap()
    }

    pub fn last_pause_time(&self) -> U256 {
        self.query_contract_dictionary(
            self.ali,
            &self.context,
            "staking_rewards_data".to_string(),
            "last_pause_time".to_string()
        ).unwrap()
    }

    pub fn paused(&self) -> bool {
        self.query_contract_dictionary(
            self.ali,
            &self.context,
            "staking_rewards_data".to_string(),
            "paused".to_string()
        ).unwrap()
    }

    pub fn owner(&self) -> AccountHash {
        self.query_contract_dictionary(
            self.ali,
            &self.context,
            "staking_rewards_data".to_string(),
            "owner".to_string()
        ).unwrap()
    }

    pub fn nominated_owner(&self) -> AccountHash {
        self.query_contract_dictionary(
            self.ali,
            &self.context,
            "staking_rewards_data".to_string(),
            "nominated_owner".to_string()
        ).unwrap()
    }
    /* ✖✖✖✖✖✖✖✖✖✖✖ Public getters - End ✖✖✖✖✖✖✖✖✖✖✖ */

    /* ✖✖✖✖✖✖✖✖✖✖✖ External functions - Start ✖✖✖✖✖✖✖✖✖✖✖ */
    pub fn set_rewards_distribution(
        &mut self,
        rewards_distribution: ContractHash,
        sender: Sender
    ) {
        self.call(
            sender,
            "set_rewards_distribution",
            runtime_args! {
                "rewards_distribution" => rewards_distribution
            },
        )
    }

    pub fn set_paused(
        &mut self,
        paused: bool,
        sender: Sender
    ) {
        self.call(
            sender,
            "set_paused",
            runtime_args! {
                "paused" => paused
            },
        )
    }

    pub fn nominate_new_owner(
        &mut self,
        owner: AccountHash,
        sender: Sender
    ) {
        self.call(
            sender,
            "nominate_new_owner",
            runtime_args! {
                "owner" => owner
            },
        )
    }

    pub fn accept_ownership(
        &mut self,
        sender: Sender
    ) {
        self.call(
            sender,
            "accept_ownership",
            runtime_args! {},
        )
    }

    pub fn get_reward_for_duration(
        &mut self,
        sender: Sender
    ) {
        self.call(
            sender,
            "get_reward_for_duration",
            runtime_args! {},
        )
    }

    pub fn stake(
        &mut self,
        amount: U256,
        sender: Sender
    ) {
        self.call(
            sender,
            "stake",
            runtime_args! {
                "amount" => amount
            },
        )
    }

    pub fn exit(
        &mut self,
        sender: Sender
    ) {
        self.call(
            sender,
            "exit",
            runtime_args! {},
        )
    }

    pub fn notify_reward_amount(
        &mut self,
        reward: U256,
        sender: Sender
    ) {
        self.call(
            sender,
            "notify_reward_amount",
            runtime_args! {
                "reward" => reward
            },
        )
    }

    pub fn update_period_finish(
        &mut self,
        timestamp: U256,
        sender: Sender
    ) {
        self.call(
            sender,
            "update_period_finish",
            runtime_args! {
                "timestamp" => timestamp
            },
        )
    }

    pub fn recover_erc20(
        &mut self,
        token_contract_hash: ContractHash,
        token_amount: U256,
        sender: Sender
    ) {
        self.call(
            sender,
            "recover_erc20",
            runtime_args! {
                "token_contract_hash" => token_contract_hash,
                "token_amount" => token_amount
            },
        )
    }

    pub fn set_rewards_duration(
        &mut self,
        rewards_duration: U256,
        sender: Sender
    ) {
        self.call(
            sender,
            "set_rewards_duration",
            runtime_args! {
                "rewards_duration" => rewards_duration
            },
        )
    }
    /* ✖✖✖✖✖✖✖✖✖✖✖ External functions - End ✖✖✖✖✖✖✖✖✖✖✖ */

}