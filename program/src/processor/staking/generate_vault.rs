use crate::consts::{ADMIN, VAULT};
use crate::error::ContractError;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction;
use solana_program::sysvar::Sysvar;

pub fn generate_vault(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    let rent = &Rent::from_account_info(accounts.rent_info)?;

    let (vault_pda, vault_bump_seed) = Pubkey::find_program_address(&[VAULT], program_id);

    if accounts.pda.key != &vault_pda {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let admin = ADMIN.parse::<Pubkey>().unwrap();

    if *accounts.payer.key != admin || !accounts.payer.is_signer {
        return Err(ContractError::UnauthorisedAccess.into());
    }

    if accounts.pda.owner != program_id {
        let required_lamports = rent
            .minimum_balance(0)
            .max(1)
            .saturating_sub(accounts.pda.lamports());

        invoke(
            &system_instruction::transfer(accounts.payer.key, &vault_pda, required_lamports),
            &[
                accounts.payer.clone(),
                accounts.pda.clone(),
                accounts.system_program.clone(),
            ],
        )?;

        invoke_signed(
            &system_instruction::assign(&vault_pda, program_id),
            &[accounts.pda.clone(), accounts.system_program.clone()],
            &[&[VAULT, &[vault_bump_seed]]],
        )?;
    }

    Ok(())
}

#[allow(dead_code)]
pub struct Accounts<'a, 'b> {
    pub payer: &'a AccountInfo<'b>,
    pub system_program: &'a AccountInfo<'b>,
    pub pda: &'a AccountInfo<'b>,
    pub rent_info: &'a AccountInfo<'b>,
}

impl<'a, 'b> Accounts<'a, 'b> {
    #[allow(dead_code)]
    pub fn new(accounts: &'a [AccountInfo<'b>]) -> Result<Accounts<'a, 'b>, ProgramError> {
        let acc_iter = &mut accounts.iter();

        Ok(Accounts {
            payer: next_account_info(acc_iter)?,
            system_program: next_account_info(acc_iter)?,
            pda: next_account_info(acc_iter)?,
            rent_info: next_account_info(acc_iter)?,
        })
    }
}
