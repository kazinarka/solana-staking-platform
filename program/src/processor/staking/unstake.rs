use crate::consts::{REWARD_MINT, VAULT, WHITELIST};
use crate::error::ContractError;
use crate::processor::staking::claim::Accounts;
use crate::state::claim::claim_transfer;
use crate::state::reward_calculation::calculate_reward;
use crate::state::structs::StakeData;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::Sysvar;

pub fn unstake(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    let clock = Clock::get()?;

    let reward_mint = REWARD_MINT.parse::<Pubkey>().unwrap();

    let (stake_address, _stake_bump) =
        Pubkey::find_program_address(&[&accounts.nft_info.key.to_bytes()], program_id);

    let (vault_address, vault_bump) = Pubkey::find_program_address(&[VAULT], program_id);

    let payer_reward_holder = spl_associated_token_account::get_associated_token_address(
        accounts.payer.key,
        &reward_mint,
    );

    let vault_reward_holder = spl_associated_token_account::get_associated_token_address(
        accounts.vault_info.key,
        &reward_mint,
    );

    let payer_nft_holder = spl_associated_token_account::get_associated_token_address(
        accounts.payer.key,
        accounts.nft_info.key,
    );

    let vault_nft_holder = spl_associated_token_account::get_associated_token_address(
        accounts.vault_info.key,
        accounts.nft_info.key,
    );

    let (metadata_address, _) = Pubkey::find_program_address(
        &[
            "metadata".as_bytes(),
            &spl_token_metadata::ID.to_bytes(),
            &accounts.nft_info.key.to_bytes(),
        ],
        &spl_token_metadata::ID,
    );

    if !accounts.payer.is_signer {
        return Err(ContractError::UnauthorisedAccess.into());
    }

    if *accounts.token_info.key != spl_token::id()
        || stake_address != *accounts.stake_info.key
        || vault_address != *accounts.vault_info.key
        || payer_reward_holder != *accounts.payer_reward_holder_info.key
        || vault_reward_holder != *accounts.vault_reward_holder_info.key
        || payer_nft_holder != *accounts.payer_nft_holder_info.key
        || vault_nft_holder != *accounts.vault_nft_holder_info.key
        || metadata_address != *accounts.metadata_info.key
        || reward_mint != *accounts.reward_mint_info.key
    {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let metadata = spl_token_metadata::state::Metadata::from_account_info(accounts.metadata_info)?;
    let creators = metadata.data.creators.unwrap();
    let creator = creators.first().unwrap();
    let creator_address = creator.address;

    let (wl_data_address, _wl_data_address_bump) =
        Pubkey::find_program_address(&[WHITELIST, &creator_address.to_bytes()], program_id);

    if *accounts.whitelist_info.key != wl_data_address {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let mut stake_data =
        if let Ok(data) = StakeData::try_from_slice(&accounts.stake_info.data.borrow()) {
            data
        } else {
            return Err(ContractError::DeserializeError.into());
        };

    if !creator.verified {
        return Err(ContractError::UnverifiedAddress.into());
    }

    if !stake_data.active {
        return Err(ContractError::InactiveStaking.into());
    }

    if stake_data.staker != *accounts.payer.key {
        return Err(ContractError::UnauthorisedAccess.into());
    }

    let reward = calculate_reward(
        clock.unix_timestamp as u64,
        stake_data.timestamp,
        stake_data.harvested,
        stake_data.withdrawn,
    );

    claim_transfer(&accounts, vault_bump, reward)?;

    if accounts.payer_nft_holder_info.owner != accounts.token_info.key {
        invoke(
            &spl_associated_token_account::create_associated_token_account(
                accounts.payer.key,
                accounts.payer.key,
                accounts.nft_info.key,
            ),
            &[
                accounts.payer.clone(),
                accounts.payer_nft_holder_info.clone(),
                accounts.payer.clone(),
                accounts.nft_info.clone(),
                accounts.system_program.clone(),
                accounts.token_info.clone(),
                accounts.rent_info.clone(),
                accounts.assoc_acccount_info.clone(),
            ],
        )?;
    }

    invoke_signed(
        &spl_token::instruction::transfer(
            accounts.token_info.key,
            accounts.vault_nft_holder_info.key,
            accounts.payer_nft_holder_info.key,
            accounts.vault_info.key,
            &[],
            1,
        )?,
        &[
            accounts.vault_nft_holder_info.clone(),
            accounts.payer_nft_holder_info.clone(),
            accounts.vault_info.clone(),
            accounts.token_info.clone(),
        ],
        &[&[VAULT, &[vault_bump]]],
    )?;

    invoke_signed(
        &spl_token::instruction::close_account(
            accounts.token_info.key,
            accounts.vault_nft_holder_info.key,
            accounts.payer.key,
            accounts.vault_info.key,
            &[],
        )?,
        &[
            accounts.vault_nft_holder_info.clone(),
            accounts.payer.clone(),
            accounts.vault_info.clone(),
            accounts.token_info.clone(),
        ],
        &[&[VAULT, &[vault_bump]]],
    )?;
    stake_data.active = false;
    stake_data.harvested += reward;
    stake_data.withdrawn += reward;
    stake_data.serialize(&mut &mut accounts.stake_info.data.borrow_mut()[..])?;

    Ok(())
}
