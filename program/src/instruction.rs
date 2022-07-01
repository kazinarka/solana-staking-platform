use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum PlatformInstruction {
    GenerateVault,
    AddToWhitelist,
    Stake,
    Stake3,
    Unstake,
    Claim,
}
