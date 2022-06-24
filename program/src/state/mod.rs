use solana_program::pubkey::Pubkey;
use borsh::{BorshDeserialize, BorshSerialize, BorshSchema};

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct StakeData{
    pub timestamp: u64,
    pub staker: Pubkey,
    pub mint: Pubkey,
    pub active: bool,
    pub withdrawn: u64,
    pub harvested: u64,
}

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct ContractData{
    pub reward_period: u64,
}


#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct RateData{
    pub price: u64,
    pub maxreward: u64,
}