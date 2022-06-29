mod staking;

use staking::admin_methods::generate_vault;
use crate::error::ContractError;
use crate::instruction::PlatformInstruction;
use borsh::BorshDeserialize;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;
use solana_program::msg;
use crate::processor::staking::admin_methods::add_to_whitelist;
use crate::processor::staking::claim::claim;
use crate::processor::staking::stake::{stake, stake5};
use crate::processor::staking::unstake::unstake;

/// Program state handler
pub struct Processor {}

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        let instruction: PlatformInstruction =
            match PlatformInstruction::try_from_slice(instruction_data) {
                Ok(insn) => insn,
                Err(err) => {
                    msg!("Failed to deserialize instruction: {}", err);
                    return Err(ContractError::InvalidInstructionData.into());
                }
            };

        match instruction {
            PlatformInstruction::GenerateVault => generate_vault(accounts_iter, program_id)?,

            PlatformInstruction::AddToWhitelist => add_to_whitelist(accounts_iter, program_id)?,

            PlatformInstruction::Stake => stake(accounts_iter, program_id)?,

            PlatformInstruction::Stake5 => stake5(accounts_iter, program_id)?,

            PlatformInstruction::Unstake => unstake(accounts_iter, program_id)?,

            PlatformInstruction::Claim => claim(accounts_iter, program_id)?,
        };

        Ok(())
    }
}
