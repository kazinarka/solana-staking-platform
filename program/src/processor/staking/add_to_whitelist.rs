use crate::consts::{ADMIN, WHITELIST};
use crate::error::ContractError;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction;
use solana_program::sysvar::Sysvar;

pub fn add_to_whitelist(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    let rent = &Rent::from_account_info(accounts.rent_info)?;

    let admin = ADMIN.parse::<Pubkey>().unwrap();

    if *accounts.payer.key != admin || !accounts.payer.is_signer {
        return Err(ContractError::UnauthorisedAccess.into());
    }

    let (data_address, data_address_bump) = Pubkey::find_program_address(
        &[WHITELIST, &accounts.creator_info.key.to_bytes()],
        program_id,
    );

    if *accounts.whitelist_info.key != data_address {
        return Err(ContractError::InvalidInstructionData.into());
    }

    if accounts.whitelist_info.owner != program_id {
        let required_lamports = rent
            .minimum_balance(0)
            .max(1)
            .saturating_sub(accounts.whitelist_info.lamports());

        invoke(
            &system_instruction::transfer(accounts.payer.key, &data_address, required_lamports),
            &[
                accounts.payer.clone(),
                accounts.whitelist_info.clone(),
                accounts.sys_info.clone(),
            ],
        )?;

        invoke_signed(
            &system_instruction::assign(&data_address, program_id),
            &[accounts.whitelist_info.clone(), accounts.sys_info.clone()],
            &[&[
                WHITELIST,
                &accounts.creator_info.key.to_bytes(),
                &[data_address_bump],
            ]],
        )?;
    }

    Ok(())
}

#[allow(dead_code)]
pub struct Accounts<'a, 'b> {
    pub payer: &'a AccountInfo<'b>,
    pub creator_info: &'a AccountInfo<'b>,
    pub whitelist_info: &'a AccountInfo<'b>,
    pub sys_info: &'a AccountInfo<'b>,
    pub rent_info: &'a AccountInfo<'b>,
}

impl<'a, 'b> Accounts<'a, 'b> {
    #[allow(dead_code)]
    pub fn new(accounts: &'a [AccountInfo<'b>]) -> Result<Accounts<'a, 'b>, ProgramError> {
        let acc_iter = &mut accounts.iter();

        Ok(Accounts {
            payer: next_account_info(acc_iter)?,
            creator_info: next_account_info(acc_iter)?,
            whitelist_info: next_account_info(acc_iter)?,
            sys_info: next_account_info(acc_iter)?,
            rent_info: next_account_info(acc_iter)?,
        })
    }
}
