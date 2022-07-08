use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct StakeData {
    pub timestamp: u64,
    pub staker: Pubkey,
    pub mint: Pubkey,
    pub active: bool,
    pub withdrawn: u64,
    pub harvested: u64,
}
