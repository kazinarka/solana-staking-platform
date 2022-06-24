use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum PlatformInstruction {
    GenerateVault,
    Stake,
    Unstake,
    Claim,
    Stake5,
    AddToWhitelist {
        #[allow(dead_code)]
        price: u64,
        #[allow(dead_code)]
        maxreward: u64,
    },
}
