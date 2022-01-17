use near_contract_standards::non_fungible_token::metadata::TokenMetadata;

use crate::common::*;

use std::collections::HashMap;

use crate::token::TokenId;

pub type TokenSeriesId = String;
pub const TOKEN_DELIMETER: char = ':';

// note, keep it all pub for now, but later switch to all private fields. 

#[derive(BorshDeserialize, BorshSerialize)]
pub struct TokenSeries {
    pub metadata: TokenMetadata,
    pub owner_id: AccountId,
    pub tokens: UnorderedSet<TokenId>,
    pub royalty: HashMap<AccountId, u32>,
    pub approved_market_id: Option<AccountId>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenSeriesJson {
    pub metadata: TokenMetadata,
    pub owner_id: AccountId,
    pub royalty: HashMap<AccountId, u32>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SeriesMintArgs {
	pub token_series_id: TokenSeriesId,
	pub receiver_id: AccountId,
}