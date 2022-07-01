use crate::consts::{VAULT, WHITELIST};
use crate::error::ContractError;
use crate::processor::staking::stake::Accounts;
use crate::state::stake::{check_metadata_account, pay_rent, transfer_nft_to_assoc};
use crate::state::structs::StakeData;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;

pub fn stake3(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    let accounts = Accounts::multiple_new(accounts)?;
    let accounts1 = accounts.get(0).unwrap();
    let accounts2 = accounts.get(1).unwrap();
    let accounts3 = accounts.get(2).unwrap();

    let clock = Clock::get()?;

    if *accounts1.token_program.key != spl_token::id() {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let rent = &Rent::from_account_info(accounts1.rent_info)?;
    let (stake_data1, stake_data_bump1) =
        Pubkey::find_program_address(&[&accounts1.mint.key.to_bytes()], &program_id);
    let (stake_data2, stake_data_bump2) =
        Pubkey::find_program_address(&[&accounts2.mint.key.to_bytes()], &program_id);
    let (stake_data3, stake_data_bump3) =
        Pubkey::find_program_address(&[&accounts3.mint.key.to_bytes()], &program_id);

    if !accounts1.payer.is_signer {
        return Err(ContractError::UnauthorisedAccess.into());
    }

    if stake_data1 != *accounts1.stake_data_info.key
        || stake_data2 != *accounts2.stake_data_info.key
        || stake_data3 != *accounts3.stake_data_info.key
    {
        return Err(ContractError::InvalidInstructionData.into());
    }

    for i in 1..4 {
        let (accounts, stake_data, stake_data_bump) = match i {
            1 => (accounts1, stake_data1, stake_data_bump1),
            2 => (accounts2, stake_data2, stake_data_bump2),
            3 => (accounts3, stake_data3, stake_data_bump3),
            _ => {
                return Err(ProgramError::Custom(101));
            }
        };

        let size: u64 = 8 + 32 + 32 + 8 + 1 + 8;
        pay_rent(
            &accounts,
            program_id,
            rent,
            size,
            stake_data,
            stake_data_bump,
        )?;

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
            Pubkey::find_program_address(&[WHITELIST, &creator_address.to_bytes()], &program_id);

        if *accounts.whitelist_info.key != wl_data_address {
            return Err(ContractError::InvalidInstructionData.into());
        }

        if accounts.whitelist_info.owner != program_id {
            return Err(ContractError::WhitelistError.into());
        }

        if !creator.verified {
            return Err(ContractError::UnverifiedAddress.into());
        }

        let (vault, _vault_bump) = Pubkey::find_program_address(&[&VAULT], &program_id);
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
    }

    Ok(())
}
