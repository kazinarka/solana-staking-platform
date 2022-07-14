use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_sdk::pubkey::Pubkey;

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum PlatformInstruction {
    GenerateVault,
    AddToWhitelist,
    Stake,
    Unstake,
    Claim,
}

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
struct StakeData {
    timestamp: u64,
    staker: Pubkey,
    mint: Pubkey,
    active: bool,
    withdrawn: u64,
    harvested: u64,
}