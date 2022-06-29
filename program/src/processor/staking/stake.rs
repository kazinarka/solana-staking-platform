use std::slice::Iter;
use solana_program::account_info::{AccountInfo, next_account_info};
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction;
use solana_program::sysvar::Sysvar;
use crate::consts::{VAULT, WHITELIST};
use crate::state::structs::StakeData;
use borsh::{BorshSerialize, BorshDeserialize};

pub fn stake(accounts_iter:&mut Iter<AccountInfo>, program_id: &Pubkey) -> ProgramResult {
    let payer = next_account_info(accounts_iter)?;
    let mint = next_account_info(accounts_iter)?;
    let metadata_account_info = next_account_info(accounts_iter)?;

    let vault_info = next_account_info(accounts_iter)?;
    let source = next_account_info(accounts_iter)?;
    let destination = next_account_info(accounts_iter)?;

    let token_program = next_account_info(accounts_iter)?;
    let sys_info = next_account_info(accounts_iter)?;
    let rent_info = next_account_info(accounts_iter)?;
    let token_assoc = next_account_info(accounts_iter)?;

    let stake_data_info = next_account_info(accounts_iter)?;
    let whitelist_info = next_account_info(accounts_iter)?;

    let clock = Clock::get()?;

    if *token_program.key != spl_token::id() {
        //wrong token_info
        return Err(ProgramError::Custom(0x345));
    }

    let rent = &Rent::from_account_info(rent_info)?;

    let (stake_data, stake_data_bump) =
        Pubkey::find_program_address(&[&mint.key.to_bytes()], &program_id);

    if !payer.is_signer {
        //unauthorized access
        return Err(ProgramError::Custom(0x11));
    }

    if stake_data != *stake_data_info.key {
        //msg!("invalid stake_data account!");
        return Err(ProgramError::Custom(0x10));
    }

    let size: u64 = 8 + 32 + 32 + 8 + 1 + 8;
    if stake_data_info.owner != program_id {
        let required_lamports = rent
            .minimum_balance(size as usize)
            .max(1)
            .saturating_sub(stake_data_info.lamports());

        invoke(
            &system_instruction::transfer(payer.key, &stake_data, required_lamports),
            &[payer.clone(), stake_data_info.clone(), sys_info.clone()],
        )?;

        invoke_signed(
            &system_instruction::allocate(&stake_data, size),
            &[stake_data_info.clone(), sys_info.clone()],
            &[&[&mint.key.to_bytes(), &[stake_data_bump]]],
        )?;

        invoke_signed(
            &system_instruction::assign(&stake_data, program_id),
            &[stake_data_info.clone(), sys_info.clone()],
            &[&[&mint.key.to_bytes(), &[stake_data_bump]]],
        )?;
    }

    let harvested =
        if let Ok(data) = StakeData::try_from_slice(&stake_data_info.data.borrow()) {
            data.harvested
        } else {
            0
        };

    let stake_struct = StakeData {
        timestamp: clock.unix_timestamp as u64,
        staker: *payer.key,
        harvested: harvested,
        active: true,
        withdrawn: 0,
        mint: *mint.key,
    };
    stake_struct.serialize(&mut &mut stake_data_info.data.borrow_mut()[..])?;

    if &Pubkey::find_program_address(
        &[
            "metadata".as_bytes(),
            &spl_token_metadata::ID.to_bytes(),
            &mint.key.to_bytes(),
        ],
        &spl_token_metadata::ID,
    )
        .0 != metadata_account_info.key
    {
        //msg!("invalid metadata account!");
        return Err(ProgramError::Custom(0x03));
    }

    let metadata =
        spl_token_metadata::state::Metadata::from_account_info(metadata_account_info)?;
    let creators = metadata.data.creators.unwrap();
    let creator = creators.first().unwrap();
    let creator_address = creator.address;

    let (wl_data_address, _wl_data_address_bump) = Pubkey::find_program_address(
        &[WHITELIST, &creator_address.to_bytes()],
        &program_id,
    );

    if *whitelist_info.key != wl_data_address {
        // wrong whitelist_info
        return Err(ProgramError::Custom(0x900));
    }

    if whitelist_info.owner != program_id {
        // nft is not whitelisted
        return Err(ProgramError::Custom(0x902));
    }

    if !creator.verified {
        //msg!("address is not verified");
        return Err(ProgramError::Custom(0x06));
    }

    let (vault, _vault_bump) = Pubkey::find_program_address(&[&VAULT], &program_id);
    if vault != *vault_info.key {
        //msg!("Wrong vault");
        return Err(ProgramError::Custom(0x07));
    }

    if &spl_associated_token_account::get_associated_token_address(payer.key, mint.key)
        != source.key
    {
        // msg!("Wrong source");
        return Err(ProgramError::Custom(0x08));
    }

    if &spl_associated_token_account::get_associated_token_address(&vault, mint.key)
        != destination.key
    {
        //msg!("Wrong destination");
        return Err(ProgramError::Custom(0x09));
    }

    if destination.owner != token_program.key {
        invoke(
            &spl_associated_token_account::create_associated_token_account(
                payer.key,
                vault_info.key,
                mint.key,
            ),
            &[
                payer.clone(),
                destination.clone(),
                vault_info.clone(),
                mint.clone(),
                sys_info.clone(),
                token_program.clone(),
                rent_info.clone(),
                token_assoc.clone(),
            ],
        )?;
    }

    invoke(
        &spl_token::instruction::transfer(
            token_program.key,
            source.key,
            destination.key,
            payer.key,
            &[],
            1,
        )?,
        &[
            source.clone(),
            destination.clone(),
            payer.clone(),
            token_program.clone(),
        ],
    )?;

    Ok(())
}

pub fn stake5(accounts_iter:&mut Iter<AccountInfo>, program_id: &Pubkey) -> ProgramResult {
    let payer = next_account_info(accounts_iter)?;
    let mint1 = next_account_info(accounts_iter)?;
    let metadata_account_info1 = next_account_info(accounts_iter)?;
    let mint2 = next_account_info(accounts_iter)?;
    let metadata_account_info2 = next_account_info(accounts_iter)?;
    let mint3 = next_account_info(accounts_iter)?;
    let metadata_account_info3 = next_account_info(accounts_iter)?;
    let mint4 = next_account_info(accounts_iter)?;
    let metadata_account_info4 = next_account_info(accounts_iter)?;
    let mint5 = next_account_info(accounts_iter)?;
    let metadata_account_info5 = next_account_info(accounts_iter)?;

    let vault_info = next_account_info(accounts_iter)?;
    let source1 = next_account_info(accounts_iter)?;
    let destination1 = next_account_info(accounts_iter)?;
    let source2 = next_account_info(accounts_iter)?;
    let destination2 = next_account_info(accounts_iter)?;
    let source3 = next_account_info(accounts_iter)?;
    let destination3 = next_account_info(accounts_iter)?;
    let source4 = next_account_info(accounts_iter)?;
    let destination4 = next_account_info(accounts_iter)?;
    let source5 = next_account_info(accounts_iter)?;
    let destination5 = next_account_info(accounts_iter)?;

    let token_program = next_account_info(accounts_iter)?;
    let sys_info = next_account_info(accounts_iter)?;
    let rent_info = next_account_info(accounts_iter)?;
    let token_assoc = next_account_info(accounts_iter)?;

    let stake_data_info1 = next_account_info(accounts_iter)?;
    let stake_data_info2 = next_account_info(accounts_iter)?;
    let stake_data_info3 = next_account_info(accounts_iter)?;
    let stake_data_info4 = next_account_info(accounts_iter)?;
    let stake_data_info5 = next_account_info(accounts_iter)?;

    let whitelist_info = next_account_info(accounts_iter)?;

    let clock = Clock::get()?;

    if *token_program.key != spl_token::id() {
        //wrong token_info
        return Err(ProgramError::Custom(0x345));
    }

    let rent = &Rent::from_account_info(rent_info)?;
    let (stake_data1, stake_data_bump1) =
        Pubkey::find_program_address(&[&mint1.key.to_bytes()], &program_id);
    let (stake_data2, stake_data_bump2) =
        Pubkey::find_program_address(&[&mint2.key.to_bytes()], &program_id);
    let (stake_data3, stake_data_bump3) =
        Pubkey::find_program_address(&[&mint3.key.to_bytes()], &program_id);
    let (stake_data4, stake_data_bump4) =
        Pubkey::find_program_address(&[&mint4.key.to_bytes()], &program_id);
    let (stake_data5, stake_data_bump5) =
        Pubkey::find_program_address(&[&mint5.key.to_bytes()], &program_id);

    if !payer.is_signer {
        //unauthorized access
        return Err(ProgramError::Custom(0x11));
    }

    if stake_data1 != *stake_data_info1.key
        || stake_data2 != *stake_data_info2.key
        || stake_data3 != *stake_data_info3.key
        || stake_data4 != *stake_data_info4.key
        || stake_data5 != *stake_data_info5.key
    {
        //msg!("invalid stake_data account!");
        return Err(ProgramError::Custom(0x10));
    }

    for i in 1..6 {
        let (
            stake_data,
            stake_data_info,
            mint,
            stake_data_bump,
            metadata_account_info,
            source,
            destination,
        ) = match i {
            1 => (
                stake_data1,
                stake_data_info1,
                mint1,
                stake_data_bump1,
                metadata_account_info1,
                source1,
                destination1,
            ),
            2 => (
                stake_data2,
                stake_data_info2,
                mint2,
                stake_data_bump2,
                metadata_account_info2,
                source2,
                destination2,
            ),
            3 => (
                stake_data3,
                stake_data_info3,
                mint3,
                stake_data_bump3,
                metadata_account_info3,
                source3,
                destination3,
            ),
            4 => (
                stake_data4,
                stake_data_info4,
                mint4,
                stake_data_bump4,
                metadata_account_info4,
                source4,
                destination4,
            ),
            5 => (
                stake_data5,
                stake_data_info5,
                mint5,
                stake_data_bump5,
                metadata_account_info5,
                source5,
                destination5,
            ),
            _ => {
                return Err(ProgramError::Custom(101));
            }
        };

        let size: u64 = 8 + 32 + 32 + 8 + 1 + 8;
        if stake_data_info.owner != program_id {
            let required_lamports = rent
                .minimum_balance(size as usize)
                .max(1)
                .saturating_sub(stake_data_info.lamports());

            invoke(
                &system_instruction::transfer(
                    payer.key,
                    &stake_data,
                    required_lamports,
                ),
                &[payer.clone(), stake_data_info.clone(), sys_info.clone()],
            )?;

            invoke_signed(
                &system_instruction::allocate(&stake_data, size),
                &[stake_data_info.clone(), sys_info.clone()],
                &[&[&mint.key.to_bytes(), &[stake_data_bump]]],
            )?;

            invoke_signed(
                &system_instruction::assign(&stake_data, program_id),
                &[stake_data_info.clone(), sys_info.clone()],
                &[&[&mint.key.to_bytes(), &[stake_data_bump]]],
            )?;
        }

        let harvested = if let Ok(data) =
        StakeData::try_from_slice(&stake_data_info.data.borrow())
        {
            data.harvested
        } else {
            0
        };

        let stake_struct = StakeData {
            timestamp: clock.unix_timestamp as u64,
            staker: *payer.key,
            harvested: harvested,
            active: true,
            withdrawn: 0,
            mint: *mint.key,
        };
        stake_struct.serialize(&mut &mut stake_data_info.data.borrow_mut()[..])?;

        if &Pubkey::find_program_address(
            &[
                "metadata".as_bytes(),
                &spl_token_metadata::ID.to_bytes(),
                &mint.key.to_bytes(),
            ],
            &spl_token_metadata::ID,
        )
            .0 != metadata_account_info.key
        {
            //msg!("invalid metadata account!");
            return Err(ProgramError::Custom(0x03));
        }

        let metadata = spl_token_metadata::state::Metadata::from_account_info(
            metadata_account_info,
        )?;
        let creators = metadata.data.creators.unwrap();
        let creator = creators.first().unwrap();
        let creator_address = creator.address;

        let (wl_data_address, _wl_data_address_bump) = Pubkey::find_program_address(
            &[WHITELIST, &creator_address.to_bytes()],
            &program_id,
        );

        if *whitelist_info.key != wl_data_address {
            // wrong whitelist_info
            return Err(ProgramError::Custom(0x900));
        }

        if whitelist_info.owner != program_id {
            // nft is not whitelisted
            return Err(ProgramError::Custom(0x902));
        }

        if !creator.verified {
            //msg!("address is not verified");
            return Err(ProgramError::Custom(0x06));
        }

        let (vault, _vault_bump) = Pubkey::find_program_address(&[&VAULT], &program_id);
        if vault != *vault_info.key {
            //msg!("Wrong vault");
            return Err(ProgramError::Custom(0x07));
        }

        if &spl_associated_token_account::get_associated_token_address(
            payer.key, mint.key,
        ) != source.key
        {
            // msg!("Wrong source");
            return Err(ProgramError::Custom(0x08));
        }

        if &spl_associated_token_account::get_associated_token_address(&vault, mint.key)
            != destination.key
        {
            //msg!("Wrong destination");
            return Err(ProgramError::Custom(0x09));
        }

        if destination.owner != token_program.key {
            invoke(
                &spl_associated_token_account::create_associated_token_account(
                    payer.key,
                    vault_info.key,
                    mint.key,
                ),
                &[
                    payer.clone(),
                    destination.clone(),
                    vault_info.clone(),
                    mint.clone(),
                    sys_info.clone(),
                    token_program.clone(),
                    rent_info.clone(),
                    token_assoc.clone(),
                ],
            )?;
        }

        invoke(
            &spl_token::instruction::transfer(
                token_program.key,
                source.key,
                destination.key,
                payer.key,
                &[],
                1,
            )?,
            &[
                source.clone(),
                destination.clone(),
                payer.clone(),
                token_program.clone(),
            ],
        )?;
    }

    Ok(())
}