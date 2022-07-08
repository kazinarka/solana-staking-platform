use crate::consts::VAULT;
use crate::processor::staking::claim::Accounts;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};

pub fn claim_transfer(accounts: &Accounts, vault_bump: u8, reward: u64) -> ProgramResult {
    if accounts.payer_reward_holder_info.owner != accounts.token_info.key {
        invoke(
            &spl_associated_token_account::create_associated_token_account(
                accounts.payer.key,
                accounts.payer.key,
                accounts.reward_mint_info.key,
            ),
            &[
                accounts.payer.clone(),
                accounts.payer_reward_holder_info.clone(),
                accounts.payer.clone(),
                accounts.reward_mint_info.clone(),
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
            accounts.vault_reward_holder_info.key,
            accounts.payer_reward_holder_info.key,
            accounts.vault_info.key,
            &[],
            reward,
        )?,
        &[
            accounts.vault_reward_holder_info.clone(),
            accounts.payer_reward_holder_info.clone(),
            accounts.vault_info.clone(),
            accounts.token_info.clone(),
        ],
        &[&[VAULT, &[vault_bump]]],
    )?;

    Ok(())
}
