use anyhow;
use csv::Writer;
use std::{env, fs::File};

use solana_account_decoder::{UiAccountEncoding, UiDataSliceConfig};
use solana_client::{
    rpc_client::RpcClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType},
};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};

use crate::model::{CsvRow, TokenUserData};

pub fn fetch_token_users(
    client: &RpcClient,
    mint: Pubkey,
    mint_decimals: u8,
    csv_file_path: &str,
) -> anyhow::Result<()> {
    let rpc_config = RpcProgramAccountsConfig {
        filters: Some(vec![
            RpcFilterType::DataSize(165),
            RpcFilterType::Memcmp(Memcmp::new(
                0,                                                   // offset
                MemcmpEncodedBytes::Bytes(mint.to_bytes().to_vec()), // encoded bytes
            )),
        ]),
        account_config: RpcAccountInfoConfig {
            commitment: Some(CommitmentConfig::confirmed()),
            encoding: Some(UiAccountEncoding::Base64),
            data_slice: Some(UiDataSliceConfig {
                offset: 32,
                length: 32 + 8,
            }),
            ..Default::default()
        },
        with_context: Some(false),
    };

    println!("Fetching {} accounts...", mint);
    let response = client.get_program_accounts_with_config(&spl_token::ID, rpc_config)?;
    println!("Fetched account: {}", response.len());

    println!("Processing and filtering data...");
    let mut filtered = response
        .into_iter()
        .map(|(pubkey, account)| {
            assert!(account.data.len() == 40);
            let owner = Pubkey::new_from_array(account.data[0..32].try_into().unwrap());
            // println!("owner: {}", owner.to_string());
            let amount = u64::from_le_bytes(account.data[32..40].try_into().unwrap());
            // println!("amount: {}", amount);
            let amount = spl_token::amount_to_ui_amount(amount, mint_decimals);

            let data = TokenUserData {
                account: pubkey,
                owner,
                amount,
            };
            return data;
        })
        .filter(|data| Pubkey::is_on_curve(&data.owner) && data.amount > 0.0)
        .collect::<Vec<TokenUserData>>();

    println!("Sorting data...");
    filtered.sort_by(|a, b| b.amount.total_cmp(&a.amount));

    let first_2000 = filtered
        .into_iter()
        .take(2000)
        .map(|val| CsvRow {
            account: val.account.to_string(),
            amount: val.amount,
            owner: val.owner.to_string(),
        })
        .collect::<Vec<CsvRow>>();

    println!("Writing data to csv file...");

    let path_buf = env::current_dir()?.join("outputs").join(csv_file_path);

    let mut wtr = Writer::from_path(path_buf.as_path())?;

    for item in first_2000 {
        wtr.serialize(item)?;
    }
    println!("Written data to csv.");

    Ok(())
}