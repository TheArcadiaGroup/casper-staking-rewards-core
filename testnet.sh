#!/bin/bash
# Global Variables
yellow=`tput setaf 3`
green=`tput setaf 4`
purple=`tput setaf 5`
red=`tput setaf 1`
reset=`tput sgr0`
white_bg=`tput setab 0`
red_bg=`tput setab 1`
NODE_ADDRESS=http://135.181.162.15:7777
DEPLOY_AMOUNT=200000000000
QUERY_AMOUNT=1921300000
STAKING_TOKEN_HASH=hash-a51f4cb335b9184c871fa3d9196d5099618e4ba821cce9924884aad17e7ced82
REWARDS_TOKEN_HASH=hash-01f145f924822d8ec110e5efa24fef8b364223fb973f0591c9da58522f1f300b
REWARDS_DISTRIBUTION_HASH=hash-06fcdfae0615de0db267c4677c0eeba3332b8c29c0b95a220fba4b15294ae5b6
STAKING_REWARDS_HASH=hash-6e072aeda51112990ca597489443e47f0867265ffa8f9ebeec8b93d0da9f1f4f
ERC20_SESSION_PATH=./target/wasm32-unknown-unknown/release/erc20.wasm
STAKING_REWARDS_SESSION_PATH=./target/wasm32-unknown-unknown/release/staking_rewards.wasm
# The following variable should be modified each time you want to change the sender of the deploy (transaction).
SENDER_KEY=./keys/staking_rewards/secret_key.pem
if [[ $1 == 'erc20' ]]
then
  if [[ $2 == 'deploy' ]]
  then
    if [[ $3 == '' ]]
    then
      echo "${red_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ Invalid Syntax! ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
      echo "[✔] ${red}erc20 ${purple}deploy${reset} <TOKEN>"
      echo "[i] ${yellow}<TOKEN> ∈ {rewards_distribution, staking_token, rewards_token}${reset}"
      exit 0
    fi
    pushd ./erc20
    cargo build --release
    popd
    if [[ $3 == 'rewards_distribution' ]]
    then
      casper-client put-deploy --chain-name casper-test --node-address ${NODE_ADDRESS} --payment-amount ${DEPLOY_AMOUNT} --session-path ${ERC20_SESSION_PATH} --secret-key ./keys/$3/secret_key.pem --session-arg "token_name:string='RewardsDistribution'" "token_symbol:string='RWDD'" "token_decimals:u8='8'" "token_total_supply:u256='1000'"
    elif [[ $3 == 'staking_token' ]]
    then
      casper-client put-deploy --chain-name casper-test --node-address ${NODE_ADDRESS} --payment-amount ${DEPLOY_AMOUNT} --session-path ${ERC20_SESSION_PATH} --secret-key ./keys/$3/secret_key.pem --session-arg "token_name:string='StakingToken'" "token_symbol:string='STK'" "token_decimals:u8='8'" "token_total_supply:u256='1000'"
    elif [[ $3 == 'rewards_token' ]]
    then
      casper-client put-deploy --chain-name casper-test --node-address ${NODE_ADDRESS} --payment-amount ${DEPLOY_AMOUNT} --session-path ${ERC20_SESSION_PATH} --secret-key ./keys/$3/secret_key.pem --session-arg "token_name:string='RewardsToken'" "token_symbol:string='RWDT'" "token_decimals:u8='8'" "token_total_supply:u256='1000'"
    else
      echo "${red_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ Invalid Token! ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
      echo "[i] ${yellow}<TOKEN> ∈ {rewards_distribution, staking_token, rewards_token}${reset}"
      exit 0
    fi
  elif [[ $2 == 'query' ]]
  then
    session_hash=${REWARDS_DISTRIBUTION_HASH}
    if [[ $3 == '' || $4 == '' ]]
    then
      echo "${red_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ Invalid Syntax! ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
      echo "[✔] ${red}erc20 ${purple}query${reset} <TOKEN> <ENDPOINT>"
      echo "[i] ${yellow}<TOKEN> ∈ {rewards_distribution, staking_token, rewards_token}${reset}"
      echo "[i] ${yellow}<ENDPOINT> ∈ {transfer, transfer_from, approve}${reset}"
      exit 0
    elif [[ $3 == 'staking_token' ]]
    then 
      session_hash=${STAKING_TOKEN_HASH}
    elif [[ $3 == 'rewards_token' ]]
    then 
      session_hash=${REWARDS_TOKEN_HASH}
    fi
    if [[ $4 == 'approve' ]]
    then
      if [[ $5 == '' || $6 == '' ]]
      then
        echo "${red_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ Invalid Syntax! ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
        echo "[✔] ${red}erc20 ${purple}query${reset} <TOKEN> ${green}approve${reset} <SPENDER> <AMOUNT>"
        exit 0
      else
        casper-client put-deploy --chain-name casper-test --node-address ${NODE_ADDRESS} --payment-amount ${QUERY_AMOUNT} --secret-key ${SENDER_KEY} --session-hash ${session_hash} --session-entry-point approve --session-arg "spender:key='$5'" "amount:u256='$6'"
      fi
    elif [[ $4 == 'transfer' ]]
    then
      if [[ $5 == '' || $6 == '' ]]
      then
        echo "${red_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ Invalid Syntax! ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
        echo "[✔] ${red}erc20 ${purple}query${reset} <TOKEN> ${green}transfer${reset} <RECIPIENT> <AMOUNT>"
        exit 0
      else
        casper-client put-deploy --chain-name casper-test --node-address ${NODE_ADDRESS} --payment-amount ${QUERY_AMOUNT} --secret-key ${SENDER_KEY} --session-hash ${session_hash} --session-entry-point transfer --session-arg "recipient:key='$5'" "amount:u256='$6'"
      fi
    elif [[ $4 == 'transfer_from' ]]
    then
      if [[ $5 == '' || $6 == '' || $7 == '' ]]
      then
        echo "${red_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ Invalid Syntax! ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
        echo "[✔] ${red}erc20 ${purple}query${reset} <TOKEN> ${green}transfer_from${reset} <OWNER> <RECIPIENT> <AMOUNT>"
        exit 0
      else
        casper-client put-deploy --chain-name casper-test --node-address ${NODE_ADDRESS} --payment-amount ${QUERY_AMOUNT} --secret-key ${SENDER_KEY} --session-hash ${session_hash} --session-entry-point transfer_from --session-arg "owner:key='$5'" "recipient:key='$6'" "amount:u256='$7'"
      fi
    fi 
  fi
elif [[ $1 == 'staking_rewards' ]]
then
  if [[ $2 == 'deploy' ]]
  then
    if [[ $3 != '' ]]
    then
        pushd ./staking-rewards
        cargo build --release
        popd
        casper-client put-deploy --chain-name casper-test --node-address ${NODE_ADDRESS} --payment-amount ${DEPLOY_AMOUNT} --session-path ${STAKING_REWARDS_SESSION_PATH} --secret-key ${SENDER_KEY} --session-arg "owner:account_hash='$3'" "rewards_distribution:key='${REWARDS_DISTRIBUTION_HASH}'" "rewards_token:key='${REWARDS_TOKEN_HASH}'" "staking_token:key='${STAKING_TOKEN_HASH}'"
    else
        echo "${red_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ Invalid Syntax! ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
        echo "[✔] ${red}staking_rewards ${purple}deploy${reset} <OWNER>"
        exit 0
    fi
  elif [[ $2 == 'query' ]]
  then
    #   ====Getters (endpoints that return data using ret()) will generate a Return Error when being queried on testnet.====
    #   ====They were tested locally and since the tests on testnet went smoothly, they should work perfectly.====
    if [[ $3 == 'set_rewards_distribution' ]]
    then
      if [[ $4 != '' ]]
      then
        casper-client put-deploy --chain-name casper-test --node-address ${NODE_ADDRESS} --payment-amount ${QUERY_AMOUNT} --secret-key ${SENDER_KEY} --session-hash ${STAKING_REWARDS_HASH} --session-entry-point set_rewards_distribution --session-arg "rewards_distribution:key='$4'"
      else
        echo "${red_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ Invalid Syntax! ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
        echo "[✔] ${red}staking_rewards ${purple}query ${green}set_rewards_distribution${reset} <REWARDS_DISTRIBUTION_KEY>"
        exit 0
      fi
    elif [[ $3 == 'set_paused' ]]
    then
      if [[ $4 != '' ]]
      then
        casper-client put-deploy --chain-name casper-test --node-address ${NODE_ADDRESS} --payment-amount ${QUERY_AMOUNT} --secret-key ${SENDER_KEY} --session-hash ${STAKING_REWARDS_HASH} --session-entry-point set_paused --session-arg "paused:bool='$4'"
      else
        echo "${red_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ Invalid Syntax! ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
        echo "[✔] ${red}staking_rewards ${purple}query ${green}set_paused${reset} <BOOL>"
        exit 0
      fi
    elif [[ $3 == 'nominate_new_owner' ]]
    then
      if [[ $4 != '' ]]
      then
        casper-client put-deploy --chain-name casper-test --node-address ${NODE_ADDRESS} --payment-amount ${QUERY_AMOUNT} --secret-key ${SENDER_KEY} --session-hash ${STAKING_REWARDS_HASH} --session-entry-point nominate_new_owner --session-arg "owner:account_hash='$4'"
      else
        echo "${red_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ Invalid Syntax! ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
        echo "[✔] ${red}staking_rewards ${purple}query ${green}nominate_new_owner${reset} <NEW_OWNER>"
        exit 0
      fi
    elif [[ $3 == 'accept_ownership' ]]
    then
      casper-client put-deploy --chain-name casper-test --node-address ${NODE_ADDRESS} --payment-amount ${QUERY_AMOUNT} --secret-key ${SENDER_KEY} --session-hash ${STAKING_REWARDS_HASH} --session-entry-point accept_ownership
    elif [[ $3 == 'stake' ]]
    then
      if [[ $4 != '' ]]
      then
        casper-client put-deploy --chain-name casper-test --node-address ${NODE_ADDRESS} --payment-amount ${QUERY_AMOUNT} --secret-key ${SENDER_KEY} --session-hash ${STAKING_REWARDS_HASH} --session-entry-point stake --session-arg "amount:u256='$4'"
      else
        echo "${red_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ Invalid Syntax! ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
        echo "[✔] ${red}staking_rewards ${purple}query ${green}stake${reset} <AMOUNT>"
        exit 0
      fi
    elif [[ $3 == 'exit' ]]
    then
      casper-client put-deploy --chain-name casper-test --node-address ${NODE_ADDRESS} --payment-amount ${QUERY_AMOUNT} --secret-key ${SENDER_KEY} --session-hash ${STAKING_REWARDS_HASH} --session-entry-point exit
    elif [[ $3 == 'notify_reward_amount' ]]
    then
      if [[ $4 != '' ]]
      then
        casper-client put-deploy --chain-name casper-test --node-address ${NODE_ADDRESS} --payment-amount ${QUERY_AMOUNT} --secret-key ${SENDER_KEY} --session-hash ${STAKING_REWARDS_HASH} --session-entry-point notify_reward_amount --session-arg "reward:u256='$4'"
      else
        echo "${red_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ Invalid Syntax! ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
        echo "[✔] ${red}staking_rewards ${purple}query ${green}notify_reward_amount${reset} <REWARD>"
        exit 0
      fi
    elif [[ $3 == 'update_period_finish' ]]
    then
      if [[ $4 != '' ]]
      then
        casper-client put-deploy --chain-name casper-test --node-address ${NODE_ADDRESS} --payment-amount ${QUERY_AMOUNT} --secret-key ${SENDER_KEY} --session-hash ${STAKING_REWARDS_HASH} --session-entry-point update_period_finish --session-arg "timestamp:u256='$4'"
      else
        echo "${red_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ Invalid Syntax! ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
        echo "[✔] ${red}staking_rewards ${purple}query ${green}update_period_finish${reset} <TIMESTAMP>"
        exit 0
      fi
    elif [[ $3 == 'recover_erc20' ]]
    then
      if [[ $4 != '' && $5 != '' ]]
      then
        casper-client put-deploy --chain-name casper-test --node-address ${NODE_ADDRESS} --payment-amount ${QUERY_AMOUNT} --secret-key ${SENDER_KEY} --session-hash ${STAKING_REWARDS_HASH} --session-entry-point recover_erc20 --session-arg "token_contract_key:key='$4'" "token_amount:u256='$5'"
      else
        echo "${red_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ Invalid Syntax! ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
        echo "[✔] ${red}staking_rewards ${purple}query ${green}recover_erc20${reset} <TOKEN_CONTRACT_KEY> <TOKEN_AMOUNT>"
        exit 0
      fi
    elif [[ $3 == 'set_rewards_duration' ]]
    then
      if [[ $4 != '' ]]
      then
        casper-client put-deploy --chain-name casper-test --node-address ${NODE_ADDRESS} --payment-amount ${QUERY_AMOUNT} --secret-key ${SENDER_KEY} --session-hash ${STAKING_REWARDS_HASH} --session-entry-point set_rewards_duration --session-arg "rewards_duration:u256='$4'"
      else
        echo "${red_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ Invalid Syntax! ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
        echo "[✔] ${red}staking_rewards ${purple}query ${green}set_rewards_duration${reset} <REWARDS_DURATION>"
        exit 0
      fi
    fi
  fi
fi
if [[ $1 == 'check_status' ]]
then
  if [[ $2 == '' ]]
  then
    echo "${red_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ Invalid Syntax! ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
    echo "[✔] ${red}check_status${reset} <DEPLOY_HASH>"
    exit 0
  fi
  casper-client get-deploy --node-address ${NODE_ADDRESS} $2 > deploy_status.json
elif [[ $1 == 'get_state_root_hash' ]]
then
  casper-client get-state-root-hash --node-address ${NODE_ADDRESS} | jq -r
elif [[ $1 == 'query_state' ]]
then
  if [[ $2 == '' || $3 == '' ]]
  then
    echo "${red_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ Invalid Syntax! ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
    echo "[✔] ${red}query_state${reset} <DEPLOYER_PUBKEY> <STATE_ROOT_HASH>"
    exit 0
  fi
  casper-client query-state --node-address ${NODE_ADDRESS} -k $2 -s $3 |jq -r
elif [[ $1 == 'examples' ]]
then 
  casper-client put-deploy --show-arg-examples
elif [[ $1 == 'syntax' ]]
then
  echo "${yellow}${white_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ 𝐒𝐘𝐍𝐓𝐀𝐗 ツ ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
  echo "[✔] ${red}examples${reset}"
  echo "[✔] ${red}check_status${reset} <DEPLOY_HASH>"
  echo "[✔] ${red}get_state_root_hash${reset}"
  echo "[✔] ${red}query_state${reset} <DEPLOYER_PUBKEY> <STATE_ROOT_HASH>"
  echo "[i] ${yellow}<TOKEN> ∈ {rewards_distribution, staking_token, rewards_token}${reset}"
  echo "[✔] ${red}erc20 ${purple}deploy${reset} <TOKEN>"
  echo "[✔] ${red}erc20 ${purple}query${reset} <TOKEN> ${green}approve${reset} <SPENDER> <AMOUNT>"
  echo "[✔] ${red}erc20 ${purple}query${reset} <TOKEN> ${green}transfer${reset} <RECIPIENT> <AMOUNT>"
  echo "[✔] ${red}erc20 ${purple}query${reset} <TOKEN> ${green}transfer_from${reset} <OWNER> <RECIPIENT> <AMOUNT>"
  echo "[✔] ${red}staking_rewards ${purple}deploy${reset} <OWNER>"
  echo "[✔] ${red}staking_rewards ${purple}query ${green}set_rewards_distribution${reset} <REWARDS_DISTRIBUTION_KEY>"
  echo "[✔] ${red}staking_rewards ${purple}query ${green}set_paused${reset} <BOOL>"
  echo "[✔] ${red}staking_rewards ${purple}query ${green}nominate_new_owner${reset} <NEW_OWNER>"
  echo "[✔] ${red}staking_rewards ${purple}query ${green}accept_ownership${reset}"
  echo "[✔] ${red}staking_rewards ${purple}query ${green}stake${reset} <AMOUNT>"
  echo "[✔] ${red}staking_rewards ${purple}query ${green}exit${reset}"
  echo "[✔] ${red}staking_rewards ${purple}query ${green}notify_reward_amount${reset} <REWARD>"
  echo "[✔] ${red}staking_rewards ${purple}query ${green}update_period_finish${reset} <TIMESTAMP>"
  echo "[✔] ${red}staking_rewards ${purple}query ${green}recover_erc20${reset} <TOKEN_CONTRACT_KEY> <TOKEN_AMOUNT>"
  echo "[✔] ${red}staking_rewards ${purple}query ${green}set_rewards_duration${reset} <REWARDS_DURATION>"
fi