use crate::error::ContractError;
use crate::processor::Processor;
use num_traits::FromPrimitive;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::{entrypoint, msg};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = Processor::process(program_id, accounts, instruction_data) {
        if let ProgramError::Custom(code) = error {
            msg!("Custom error: {:?} ", ContractError::from_u32(code));
        }
        return Err(error);
    }

    Ok(())
}
