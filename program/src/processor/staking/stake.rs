use crate::consts::{VAULT, WHITELIST};
use crate::error::ContractError;
use crate::state::stake::{check_metadata_account, pay_rent, transfer_nft_to_assoc};
use crate::state::structs::StakeData;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;

pub fn stake(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    let clock = Clock::get()?;

    if *accounts.token_program.key != spl_token::id() {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let rent = &Rent::from_account_info(accounts.rent_info)?;

    let (stake_data, stake_data_bump) =
        Pubkey::find_program_address(&[&accounts.mint.key.to_bytes()], program_id);

    if !accounts.payer.is_signer {
        return Err(ContractError::UnauthorisedAccess.into());
    }

    if stake_data != *accounts.stake_data_info.key {
        return Err(ContractError::InvalidInstructionData.into());
    }

    pay_rent(&accounts, program_id, rent, stake_data, stake_data_bump)?;

    let harvested =
        if let Ok(data) = StakeData::try_from_slice(&accounts.stake_data_info.data.borrow()) {
            data.harvested
        } else {
            0
        };

    let stake_struct = StakeData {
        timestamp: clock.unix_timestamp as u64,
        staker: *accounts.payer.key,
        harvested,
        active: true,
        withdrawn: 0,
        mint: *accounts.mint.key,
    };
    stake_struct.serialize(&mut &mut accounts.stake_data_info.data.borrow_mut()[..])?;

    check_metadata_account(accounts.mint, accounts.metadata_account_info)?;

    let metadata =
        spl_token_metadata::state::Metadata::from_account_info(accounts.metadata_account_info)?;
    let creators = metadata.data.creators.unwrap();
    let creator = creators.first().unwrap();
    let creator_address = creator.address;

    let (wl_data_address, _wl_data_address_bump) =
        Pubkey::find_program_address(&[WHITELIST, &creator_address.to_bytes()], program_id);

    if *accounts.whitelist_info.key != wl_data_address {
        return Err(ContractError::InvalidInstructionData.into());
    }

    if accounts.whitelist_info.owner != program_id {
        return Err(ContractError::WhitelistError.into());
    }

    if !creator.verified {
        return Err(ContractError::UnverifiedAddress.into());
    }

    let (vault, _vault_bump) = Pubkey::find_program_address(&[VAULT], program_id);

    if vault != *accounts.vault_info.key {
        return Err(ContractError::InvalidInstructionData.into());
    }

    if &spl_associated_token_account::get_associated_token_address(
        accounts.payer.key,
        accounts.mint.key,
    ) != accounts.source.key
    {
        return Err(ContractError::InvalidInstructionData.into());
    }

    if &spl_associated_token_account::get_associated_token_address(&vault, accounts.mint.key)
        != accounts.destination.key
    {
        return Err(ContractError::InvalidInstructionData.into());
    }

    transfer_nft_to_assoc(&accounts)?;

    Ok(())
}

#[allow(dead_code)]
pub struct Accounts<'a, 'b> {
    pub payer: &'a AccountInfo<'b>,
    pub mint: &'a AccountInfo<'b>,
    pub metadata_account_info: &'a AccountInfo<'b>,
    pub vault_info: &'a AccountInfo<'b>,
    pub source: &'a AccountInfo<'b>,
    pub destination: &'a AccountInfo<'b>,
    pub token_program: &'a AccountInfo<'b>,
    pub sys_info: &'a AccountInfo<'b>,
    pub rent_info: &'a AccountInfo<'b>,
    pub token_assoc: &'a AccountInfo<'b>,
    pub stake_data_info: &'a AccountInfo<'b>,
    pub whitelist_info: &'a AccountInfo<'b>,
}

impl<'a, 'b> Accounts<'a, 'b> {
    #[allow(dead_code)]
    pub fn new(accounts: &'a [AccountInfo<'b>]) -> Result<Accounts<'a, 'b>, ProgramError> {
        let acc_iter = &mut accounts.iter();

        Ok(Accounts {
            payer: next_account_info(acc_iter)?,
            mint: next_account_info(acc_iter)?,
            metadata_account_info: next_account_info(acc_iter)?,
            vault_info: next_account_info(acc_iter)?,
            source: next_account_info(acc_iter)?,
            destination: next_account_info(acc_iter)?,
            token_program: next_account_info(acc_iter)?,
            sys_info: next_account_info(acc_iter)?,
            rent_info: next_account_info(acc_iter)?,
            token_assoc: next_account_info(acc_iter)?,
            stake_data_info: next_account_info(acc_iter)?,
            whitelist_info: next_account_info(acc_iter)?,
        })
    }
}
