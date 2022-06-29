use std::slice::Iter;
use solana_program::account_info::{AccountInfo, next_account_info};
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::Sysvar;
use crate::consts::{REWARD_MINT, VAULT, WHITELIST};
use crate::state::reward_calculation::calculate_reward;
use crate::state::structs::StakeData;
use borsh::{BorshSerialize, BorshDeserialize};

pub fn claim(accounts_iter:&mut Iter<AccountInfo>, program_id: &Pubkey) -> ProgramResult {
    let payer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let nft_info = next_account_info(accounts_iter)?;
    let token_info = next_account_info(accounts_iter)?;
    let rent_info = next_account_info(accounts_iter)?;
    let assoc_acccount_info = next_account_info(accounts_iter)?;
    let stake_info = next_account_info(accounts_iter)?;
    let vault_info = next_account_info(accounts_iter)?;
    let payer_reward_holder_info = next_account_info(accounts_iter)?;
    let vault_reward_holder_info = next_account_info(accounts_iter)?;
    let payer_nft_holder_info = next_account_info(accounts_iter)?;
    let vault_nft_holder_info = next_account_info(accounts_iter)?;
    let metadata_info = next_account_info(accounts_iter)?;

    let whitelist_info = next_account_info(accounts_iter)?;
    let reward_mint_info = next_account_info(accounts_iter)?;

    let clock = Clock::get()?;

    let reward_mint = REWARD_MINT.parse::<Pubkey>().unwrap();

    let (stake_address, _stake_bump) =
        Pubkey::find_program_address(&[&nft_info.key.to_bytes()], &program_id);
    let (vault_address, vault_bump) =
        Pubkey::find_program_address(&[&VAULT], &program_id);
    let payer_reward_holder =
        spl_associated_token_account::get_associated_token_address(
            payer.key,
            &reward_mint,
        );
    let vault_reward_holder =
        spl_associated_token_account::get_associated_token_address(
            vault_info.key,
            &reward_mint,
        );
    let payer_nft_holder = spl_associated_token_account::get_associated_token_address(
        payer.key,
        nft_info.key,
    );
    let vault_nft_holder = spl_associated_token_account::get_associated_token_address(
        vault_info.key,
        nft_info.key,
    );
    let (metadata_address, _) = Pubkey::find_program_address(
        &[
            "metadata".as_bytes(),
            &spl_token_metadata::ID.to_bytes(),
            &nft_info.key.to_bytes(),
        ],
        &spl_token_metadata::ID,
    );

    if !payer.is_signer {
        //unauthorized access
        return Err(ProgramError::Custom(0x11));
    }

    if *token_info.key != spl_token::id() {
        //wrong token_info
        return Err(ProgramError::Custom(0x345));
    }

    if stake_address != *stake_info.key {
        //wrong stake_info
        return Err(ProgramError::Custom(0x60));
    }

    if vault_address != *vault_info.key {
        //wrong stake_info
        return Err(ProgramError::Custom(0x61));
    }

    if payer_reward_holder != *payer_reward_holder_info.key {
        //wrong payer_reward_holder_info
        return Err(ProgramError::Custom(0x62));
    }

    if vault_reward_holder != *vault_reward_holder_info.key {
        //wrong vault_reward_holder_info
        return Err(ProgramError::Custom(0x63));
    }

    if payer_nft_holder != *payer_nft_holder_info.key {
        //wrong payer_nft_holder_info
        return Err(ProgramError::Custom(0x64));
    }

    if vault_nft_holder != *vault_nft_holder_info.key {
        //wrong vault_nft_holder_info
        return Err(ProgramError::Custom(0x65));
    }

    if metadata_address != *metadata_info.key {
        //wrong metadata_info
        return Err(ProgramError::Custom(0x66));
    }

    if reward_mint != *reward_mint_info.key {
        //wrong reward_mint_info
        return Err(ProgramError::Custom(0x67));
    }

    let metadata =
        spl_token_metadata::state::Metadata::from_account_info(metadata_info)?;
    let creators = metadata.data.creators.unwrap();
    let creator = creators.first().unwrap();
    let creator_address = creator.address;

    let (wl_data_address, _wl_data_address_bump) = Pubkey::find_program_address(
        &[WHITELIST, &creator_address.to_bytes()],
        &program_id,
    );

    if *whitelist_info.key != wl_data_address {
        // wrong whitelist_info
        return Err(ProgramError::Custom(0x910));
    }

    let mut stake_data =
        if let Ok(data) = StakeData::try_from_slice(&stake_info.data.borrow()) {
            data
        } else {
            // can't deserialize stake data
            return Err(ProgramError::Custom(0x913));
        };

    if !creator.verified {
        //msg!("address is not verified");
        return Err(ProgramError::Custom(0x106));
    }

    if !stake_data.active {
        //staking is inactive
        return Err(ProgramError::Custom(0x107));
    }

    if stake_data.staker != *payer.key {
        //unauthorized access
        return Err(ProgramError::Custom(0x108));
    }

    let reward = calculate_reward(
        clock.unix_timestamp as u64,
        stake_data.timestamp,
        stake_data.harvested,
        stake_data.withdrawn,
    );

    if payer_reward_holder_info.owner != token_info.key {
        invoke(
            &spl_associated_token_account::create_associated_token_account(
                payer.key,
                payer.key,
                reward_mint_info.key,
            ),
            &[
                payer.clone(),
                payer_reward_holder_info.clone(),
                payer.clone(),
                reward_mint_info.clone(),
                system_program.clone(),
                token_info.clone(),
                rent_info.clone(),
                assoc_acccount_info.clone(),
            ],
        )?;
    }

    invoke_signed(
        &spl_token::instruction::transfer(
            token_info.key,
            vault_reward_holder_info.key,
            payer_reward_holder_info.key,
            vault_info.key,
            &[],
            reward,
        )?,
        &[
            vault_reward_holder_info.clone(),
            payer_reward_holder_info.clone(),
            vault_info.clone(),
            token_info.clone(),
        ],
        &[&[&VAULT, &[vault_bump]]],
    )?;

    stake_data.harvested += reward;
    stake_data.withdrawn += reward;
    stake_data.serialize(&mut &mut stake_info.data.borrow_mut()[..])?;

    Ok(())
}