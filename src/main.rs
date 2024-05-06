mod fetch_token_users;
mod model;

use anyhow::Ok;
use dotenv::dotenv;
use fetch_token_users::*;
use solana_sdk::pubkey::Pubkey;

use std::{env, str::FromStr};

use solana_client::rpc_client::RpcClient;

fn main() -> anyhow::Result<()> {
    dotenv().ok();

    // create rpc client similar to connection in web3.js sdk. require RPC_URL in .env file
    let rpc_url = env::var("RPC_URL").expect("Missing env var: 'RPC_URL'");
    let client = RpcClient::new(rpc_url);

    let zebec = Pubkey::from_str("zebeczgi5fSEtbpfQKVZKCJ3WgYXxjkMUkNNx7fLKAF").unwrap();
    let zebec_decimals = 9;

    let wormhole: Pubkey =
        Pubkey::from_str("85VBFQZC9TZkfaptBWjvUw7YbZjy52A6mjtPGjstQAmQ").unwrap();
    let wormhole_decimals = 6;

    let jup = Pubkey::from_str("JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN").unwrap();
    let jup_decimals = 6;

    let bonk: Pubkey = Pubkey::from_str("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263").unwrap();
    let bonk_decimals = 5;

    fetch_token_users(&client, zebec, zebec_decimals, "zebec_users.csv")?;
    fetch_token_users(&client, wormhole, wormhole_decimals, "wormhole_users.csv")?;
    fetch_token_users(&client, jup, jup_decimals, "jup_users.csv")?;
    fetch_token_users(&client, bonk, bonk_decimals, "bonk_users.csv")?;

    Ok(())
}
