use serde::Serialize;
use solana_sdk::pubkey::Pubkey;

pub struct TokenUserData {
    pub account: Pubkey,
    pub owner: Pubkey,
    pub amount: f64,
}

#[derive(Debug, Serialize)]
pub struct CsvRow {
    pub account: String,
    pub owner: String,
    pub amount: f64,
}
