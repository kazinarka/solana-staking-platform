use crate::consts::{REWARD_MINT, VAULT, WHITELIST};
use crate::error::ContractError;
use crate::state::claim::claim_transfer;
use crate::state::reward_calculation::calculate_reward;
use crate::state::stake::get_stake_data;
use borsh::BorshSerialize;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::Sysvar;

pub fn claim(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    let clock = Clock::get()?;

    let reward_mint = REWARD_MINT.parse::<Pubkey>().unwrap();

    let (stake_address, _stake_bump) =
        Pubkey::find_program_address(&[&accounts.nft_info.key.to_bytes()], &program_id);

    let (vault_address, vault_bump) = Pubkey::find_program_address(&[&VAULT], &program_id);

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
        Pubkey::find_program_address(&[WHITELIST, &creator_address.to_bytes()], &program_id);

    if *accounts.whitelist_info.key != wl_data_address {
        return Err(ContractError::InvalidInstructionData.into());
    }

    let mut stake_data = get_stake_data(&accounts.stake_info.data.borrow())?;

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

    stake_data.harvested += reward;
    stake_data.withdrawn += reward;
    stake_data.serialize(&mut &mut accounts.stake_info.data.borrow_mut()[..])?;

    Ok(())
}

#[allow(dead_code)]
pub struct Accounts<'a, 'b> {
    pub payer: &'a AccountInfo<'b>,
    pub system_program: &'a AccountInfo<'b>,
    pub nft_info: &'a AccountInfo<'b>,
    pub token_info: &'a AccountInfo<'b>,
    pub rent_info: &'a AccountInfo<'b>,
    pub assoc_acccount_info: &'a AccountInfo<'b>,
    pub stake_info: &'a AccountInfo<'b>,
    pub vault_info: &'a AccountInfo<'b>,
    pub payer_reward_holder_info: &'a AccountInfo<'b>,
    pub vault_reward_holder_info: &'a AccountInfo<'b>,
    pub payer_nft_holder_info: &'a AccountInfo<'b>,
    pub vault_nft_holder_info: &'a AccountInfo<'b>,
    pub metadata_info: &'a AccountInfo<'b>,
    pub whitelist_info: &'a AccountInfo<'b>,
    pub reward_mint_info: &'a AccountInfo<'b>,
}

impl<'a, 'b> Accounts<'a, 'b> {
    #[allow(dead_code)]
    pub fn new(accounts: &'a [AccountInfo<'b>]) -> Result<Accounts<'a, 'b>, ProgramError> {
        let acc_iter = &mut accounts.iter();

        Ok(Accounts {
            payer: next_account_info(acc_iter)?,
            system_program: next_account_info(acc_iter)?,
            nft_info: next_account_info(acc_iter)?,
            token_info: next_account_info(acc_iter)?,
            rent_info: next_account_info(acc_iter)?,
            assoc_acccount_info: next_account_info(acc_iter)?,
            stake_info: next_account_info(acc_iter)?,
            vault_info: next_account_info(acc_iter)?,
            payer_reward_holder_info: next_account_info(acc_iter)?,
            vault_reward_holder_info: next_account_info(acc_iter)?,
            payer_nft_holder_info: next_account_info(acc_iter)?,
            vault_nft_holder_info: next_account_info(acc_iter)?,
            metadata_info: next_account_info(acc_iter)?,
            whitelist_info: next_account_info(acc_iter)?,
            reward_mint_info: next_account_info(acc_iter)?,
        })
    }
}
