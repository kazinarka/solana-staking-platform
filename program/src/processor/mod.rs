pub mod staking;

use crate::error::ContractError;
use crate::instruction::PlatformInstruction;
use borsh::BorshDeserialize;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;
use solana_program::msg;
use crate::processor::staking::add_to_whitelist::add_to_whitelist;
use crate::processor::staking::claim::claim;
use crate::processor::staking::generate_vault::generate_vault;
use crate::processor::staking::stake5::stake5;
use crate::processor::staking::stake::stake;
use crate::processor::staking::unstake::unstake;

/// Program state handler
pub struct Processor {}

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction: PlatformInstruction =
            match PlatformInstruction::try_from_slice(instruction_data) {
                Ok(insn) => insn,
                Err(err) => {
                    msg!("Failed to deserialize instruction: {}", err);
                    return Err(ContractError::InvalidInstructionData.into());
                }
            };

        match instruction {
            PlatformInstruction::GenerateVault => generate_vault(accounts, program_id)?,

            PlatformInstruction::AddToWhitelist => add_to_whitelist(accounts, program_id)?,

            PlatformInstruction::Stake => stake(accounts, program_id)?,

            PlatformInstruction::Stake5 => stake5(accounts, program_id)?,

            PlatformInstruction::Unstake => unstake(accounts, program_id)?,

            PlatformInstruction::Claim => claim(accounts, program_id)?,
        };

        Ok(())
    }
}
