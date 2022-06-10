mod structs;

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo, Transfer};
use crate::structs::{StakingInstance, User};
use std::ops::Deref;

declare_id!("89hQPGoZAr3M5czpv3yrr24o8aPV2VRSKPKV8kPhxXYb");
pub static USER_SEED: &[u8] = b"user_deposit";
pub static STAKING_SEED: &[u8] = b"staking_instance";
pub static COMPUTATION_DECIMALS: u64 = 10u64.pow(12);

fn update_reward_pool(
    current_timestamp: u64,
    staking_instance: &mut StakingInstance,
    #[allow(unused_variables)]
    user_instance: &mut User,
) {
    let income = staking_instance.reward_token_per_sec
        .checked_mul(current_timestamp
            .checked_sub(staking_instance.last_reward_timestamp)
            .unwrap())
        .unwrap();
    staking_instance.accumulated_reward_per_share =
        staking_instance.accumulated_reward_per_share
            .checked_add(income.checked_mul(COMPUTATION_DECIMALS).unwrap()
                .checked_div(staking_instance.total_shares)
                .unwrap_or(0))
            .unwrap();
    staking_instance.last_reward_timestamp = current_timestamp;
}

fn store_pending_reward(
    staking_instance: &mut StakingInstance,
    user_instance: &mut User,
) {
    user_instance.accumulated_reward = user_instance.accumulated_reward
        .checked_add(user_instance.deposited_amount
            .checked_mul(staking_instance.accumulated_reward_per_share)
            .unwrap()
            .checked_div(COMPUTATION_DECIMALS)
            .unwrap()
            .checked_sub(user_instance.reward_debt)
            .unwrap())
        .unwrap();
}

fn update_reward_debt(
    staking_instance: &mut StakingInstance,
    user_instance: &mut User,
) {
    user_instance.reward_debt = user_instance.deposited_amount
        .checked_mul(staking_instance.accumulated_reward_per_share)
        .unwrap()
        .checked_div(COMPUTATION_DECIMALS)
        .unwrap();
}

#[program]
pub mod pixel_staking {

    pub static TOKEN_MINT_ADDRESS: &str = "5VduWunyoseTpQTdkmHbwwP1HVXQKJyKaQq8NYPKSMgB";
    pub static NFT_MINT_ADDRESS: &str = "J53NaX4QWzPWCHUff8upk1PbCzHZZd3W5b6Jfkq8DFsv";
    pub static TOKEN_PROGRAM_BYTES: &str = "DuxMbXyzxZszG9v1Ro2ejCkTKZn1Ytr89RCuAoido2Gj";
    pub static NFT_TOKEN_PROGRAM_BYTES: &str = "9bdA9LKCtGt1o2ZdnbBeDmyeL2jSzjHL2F6HJj1z8dut";

    use super::*;

    pub fn create_token_bag(
        _ctx: Context<CreateTokenBag>
    ) -> Result<()> {
        Ok(())
    }

    pub fn stake(
        ctx: Context<Stake>,
        stake_mint_authority_bump: u8,
        _program_token_bag_bump: u8,
        token_amount: u64
    ) -> Result<()> {

        let stake_amount = token_amount;

        let stake_mint_address= ctx.accounts.stake_mint.key();
        let seeds = &[stake_mint_address.as_ref(), &[stake_mint_authority_bump]];
        let signer = [&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.stake_mint.to_account_info(),
                to: ctx.accounts.user_stake_token_bag.to_account_info(),
                authority: ctx.accounts.stake_mint_authority.to_account_info(),
            },
            &signer
        );
        token::mint_to(cpi_ctx, stake_amount)?;

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.user_token_bag.to_account_info(),
                authority: ctx.accounts.user_token_bag_authority.to_account_info(),
                to: ctx.accounts.program_token_bag.to_account_info(),
            }
        );
        token::transfer(cpi_ctx, token_amount)?;

        Ok(())
    }


    pub fn unstake(
        ctx: Context<UnStake>,
        program_bag_bump: u8,
        stake_amount: u64
    ) -> Result<()> {

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Burn {
                mint: ctx.accounts.stake_mint.to_account_info(),
                from: ctx.accounts.user_stake_token_bag.to_account_info(),
                authority: ctx.accounts.user_stake_token_bag_authority.to_account_info(),
            },
        );
        token::burn(cpi_ctx, stake_amount)?;

        let token_mint_address= ctx.accounts.token_mint.key();
        let seeds = &[token_mint_address.as_ref(), &[program_bag_bump]];
        let signer = [&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.program_token_bag.to_account_info(),
                authority: ctx.accounts.program_token_bag.to_account_info(),
                to: ctx.accounts.user_token_bag.to_account_info()
            },
            &signer
        );

        let token_amount = stake_amount;
        token::transfer(cpi_ctx, token_amount)?;

        Ok(())
    }

    pub fn initialize_staking(
        ctx: Context<InitializeStaking>,
        token_per_sec: u64,
        _staking_instance_bump: u8,
    ) -> Result<()> {
        let staking_instance = &mut ctx.accounts.staking_instance;
        staking_instance.authority= ctx.accounts.authority.key().clone();
        staking_instance.reward_token_per_sec = token_per_sec;
        staking_instance.last_reward_timestamp = ctx.accounts.time.unix_timestamp as u64;
        staking_instance.accumulated_reward_per_share = 0;
        staking_instance.reward_token_mint = ctx
            .accounts
            .reward_token_mint
            .to_account_info()
            .key()
            .clone();
        staking_instance.allowed_collection_address = ctx
            .accounts
            .allowed_collection_address
            .key()
            .clone();
        Ok(())
    }

    pub fn initialize_user(
        ctx: Context<InitializeUser>,
        _staking_instance_bump: u8,
        _staking_user_bump: u8,
    ) -> Result<()> {
        let user_instance = &mut ctx.accounts.user_instance;
        user_instance.deposited_amount = 0;
        user_instance.reward_debt = 0;
        user_instance.accumulated_reward = 0;
        Ok(())
    }

    pub fn enter_staking(
        ctx: Context<EnterStaking>,
        _staking_instance_bump: u8,
        _staking_user_bump: u8,
    ) -> Result<()> {
        let data = &mut ctx.accounts.nft_token_metadata.try_borrow_data()?;
        let val = mpl_token_metadata::state::Metadata::deserialize(&mut &data[..])?;
        let collection_not_proper = val
            .data
            .creators
            .as_ref()
            .unwrap()
            .iter()
            .filter(|item|{
                ctx.accounts.allowed_collection_address.key() ==
                    item.address && item.verified
            })
            .count() == 0;
        if collection_not_proper || val.mint != ctx.accounts.nft_token_mint.key() {
            msg!("error");
            return Ok(());
        }
        let staking_instance = &mut ctx.accounts.staking_instance;
        let user_instance = &mut ctx.accounts.user_instance;
        let current_timestamp = ctx.accounts.time.unix_timestamp as u64;
        update_reward_pool(
            current_timestamp,
            staking_instance,
            user_instance,
        );

        let cpi_accounts = Transfer {
            to: ctx.accounts.nft_token_program_wallet.to_account_info(),
            from: ctx.accounts.nft_token_authority_wallet.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let context = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(context, 1)?;

        user_instance.deposited_amount = user_instance
            .deposited_amount
            .checked_add(1)
            .unwrap();
        staking_instance.total_shares = staking_instance
            .total_shares
            .checked_add(1)
            .unwrap();
        update_reward_debt(
            staking_instance,
            user_instance,
        );
        Ok(())
    }


    pub fn cancel_staking(
        ctx: Context<CancelStaking>,
        staking_instance_bump: u8,
        _staking_user_bump: u8,
    ) -> Result<()> {
        let data = &mut ctx.accounts.nft_token_metadata.try_borrow_data()?;
        msg!("borrow");
        let val = mpl_token_metadata::state::Metadata::deserialize(&mut &data[..])?;
        msg!("deser");
        let collection_not_proper = val
            .data
            .creators
            .as_ref()
            .unwrap()
            .iter()
            .filter(|item|{
                ctx.accounts.allowed_collection_address.key() ==
                    item.address && item.verified
            })
            .count() == 0;
        msg!("count");
        if collection_not_proper || val.mint != ctx.accounts.nft_token_mint.key() {
            msg!("error");
            return Ok(());
        }

        let staking_instance = &mut ctx.accounts.staking_instance;
        let user_instance = &mut ctx.accounts.user_instance;
        let current_timestamp = ctx.accounts.time.unix_timestamp as u64;
        msg!("get accounts");
        update_reward_pool(
            current_timestamp,
            staking_instance,
            user_instance,
        );
        msg!("upd pool");
        store_pending_reward(
            staking_instance,
            user_instance,
        );

        let cpi_accounts = Transfer {
            to: ctx.accounts.nft_token_authority_wallet.to_account_info(),
            from: ctx.accounts.nft_token_program_wallet.to_account_info(),
            authority: staking_instance.clone().to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let context = CpiContext::new(cpi_program, cpi_accounts);
        let authority_seeds = &[
            &STAKING_SEED[..],
            staking_instance.authority.as_ref(),
            &[staking_instance_bump]
        ];
        token::transfer(context.with_signer(&[&authority_seeds[..]]), 1)?;

        user_instance.deposited_amount = user_instance
            .deposited_amount
            .checked_sub(1)
            .unwrap();
        staking_instance.total_shares = staking_instance
            .total_shares
            .checked_sub(1)
            .unwrap();
        update_reward_debt(
            staking_instance,
            user_instance,
        );
        Ok(())
    }

    pub fn claim_rewards(
        ctx: Context<ClaimRewards>,
        amount: u64,
        staking_instance_bump: u8,
        _staking_user_bump: u8,
    ) -> Result<()> {
        let staking_instance = &mut ctx.accounts.staking_instance;
        let user_instance = &mut ctx.accounts.user_instance;
        let current_timestamp = ctx.accounts.time.unix_timestamp as u64;
        update_reward_pool(
            current_timestamp,
            staking_instance,
            user_instance,
        );
        store_pending_reward(
            staking_instance,
            user_instance,
        );

        let cpi_accounts = MintTo {
            mint: ctx.accounts.reward_token_mint.to_account_info(),
            to: ctx.accounts.reward_token_authority_wallet.to_account_info(),
            authority: staking_instance.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let context = CpiContext::new(cpi_program, cpi_accounts);
        let authority_seeds = &[
            &STAKING_SEED[..],
            staking_instance.authority.as_ref(),
            &[staking_instance_bump]
        ];

        let amount = if amount == 0 {
            user_instance.accumulated_reward
        } else {
            amount
        };
        user_instance.accumulated_reward = user_instance
            .accumulated_reward
            .checked_sub(amount)
            .unwrap();

        token::mint_to(context.with_signer(&[&authority_seeds[..]]), amount)?;
        update_reward_debt(
            staking_instance,
            user_instance,
        );
        Ok(())
    }
}



#[derive(Accounts)]
pub struct CreateTokenBag<'info> {
    #[account(
        init,
        payer = payer,
        seeds = [ TOKEN_MINT_ADDRESS.parse::<Pubkey>().unwrap().as_ref() ],
        bump,
        token::mint = token_mint,
        token::authority = program_token_bag,
    )]
    pub program_token_bag: Account<'info, TokenAccount>,

    #[account(
        address = TOKEN_MINT_ADDRESS.parse::<Pubkey>().unwrap(),
    )]
    pub token_mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(stake_mint_authority_bump: u8, program_token_bag_bump: u8)]
pub struct Stake<'info> {
    pub token_program: Program<'info, Token>,

    #[account(
        mut,
        address = NFT_MINT_ADDRESS.parse::<Pubkey>().unwrap(),
    )]
    pub stake_mint: Account<'info, Mint>,

    /// CHECK: only used as a signing PDA
    #[account(
        seeds = [ stake_mint.key().as_ref() ],
        bump = stake_mint_authority_bump,
    )]
    pub stake_mint_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub user_stake_token_bag: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_bag: Account<'info, TokenAccount>,

    pub user_token_bag_authority: Signer<'info>,

    #[account(
        mut,
        seeds = [ token_mint.key().as_ref() ],
        bump = program_token_bag_bump,
    )]
    pub program_token_bag: Account<'info, TokenAccount>,

    #[account(
        address = TOKEN_MINT_ADDRESS.parse::<Pubkey>().unwrap(),
    )]
    pub token_mint: Account<'info, Mint>,
}

#[derive(Accounts)]
#[instruction(program_token_bag_bump: u8)]
pub struct UnStake<'info> {
    pub token_program: Program<'info, Token>,

    #[account(
        mut,
        address = NFT_MINT_ADDRESS.parse::<Pubkey>().unwrap(),
    )]
    pub stake_mint: Account<'info, Mint>,

    #[account(mut)]
    pub user_stake_token_bag: Account<'info, TokenAccount>,

    pub user_stake_token_bag_authority: Signer<'info>,

    #[account(
        mut,
        seeds = [ token_mint.key().as_ref() ],
        bump = program_token_bag_bump,
    )]
    pub program_token_bag: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_bag: Account<'info, TokenAccount>,

    #[account(
        address = TOKEN_MINT_ADDRESS.parse::<Pubkey>().unwrap(),
    )]
    pub token_mint: Box<Account<'info, Mint>>,
}

#[derive(Accounts)]
#[instruction(
    _user_instance_bump: u8,
    _staking_instance_bump: u8,
)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [
            USER_SEED.as_ref(),
            staking_instance.key().as_ref(),
            authority.key().as_ref()
        ],
        bump,
        payer = authority,
        space = 8 + core::mem::size_of::<User>(),
    )]
    pub user_instance: Box<Account<'info, User>>,

    #[account(
        mut,
        seeds = [
            STAKING_SEED.as_ref(),
            staking_instance.authority.as_ref()
        ],
        bump = _staking_instance_bump,
    )]
    pub staking_instance: Account<'info, StakingInstance>,

    pub system_program: Program<'info, System>,

    /// CHECK
    pub rent: AccountInfo<'info>,

    pub time: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
#[instruction(
    _staking_instance_bump: u8,
    token_per_sec: u64,
)]
pub struct InitializeStaking<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        constraint = reward_token_mint.mint_authority.unwrap() == staking_instance.key(),
    )]
    pub reward_token_mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        seeds = [crate::STAKING_SEED.as_ref(),authority.key().as_ref()],
        bump,
        space = 8 + core::mem::size_of::<StakingInstance>(),
        payer = authority,
    )]
    pub staking_instance: Account<'info, StakingInstance>,

    /// CHECK
    pub allowed_collection_address: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    /// CHECK
    pub rent: AccountInfo<'info>,

    pub time: Sysvar<'info,Clock>,
}

#[derive(Accounts)]
#[instruction(
    _staking_instance_bump: u8,
    _staking_user_bump: u8,
)]
pub struct EnterStaking<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        constraint = reward_token_mint
            .mint_authority
            .unwrap()
            .eq(&staking_instance.key())
    )]
    pub reward_token_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub nft_token_mint: Box<Account<'info, Mint>>,

    #[account(
        constraint = nft_token_metadata.owner == &nft_program_id.key(),
    )]
    /// CHECK
    pub nft_token_metadata: AccountInfo<'info>,

    #[account(
        mut,
        constraint = nft_token_authority_wallet
            .clone().into_inner().deref().owner == authority.key(),
        constraint = nft_token_authority_wallet
            .clone().into_inner().deref().mint == nft_token_mint.key(),
    )]
    pub nft_token_authority_wallet: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = nft_token_program_wallet
            .clone().into_inner().deref().owner == staking_instance.key(),
        constraint = nft_token_program_wallet
            .clone().into_inner().deref().mint == nft_token_mint.key(),
    )]
    pub nft_token_program_wallet: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [crate::STAKING_SEED.as_ref(), staking_instance.authority.as_ref()],
        bump = _staking_instance_bump,
    )]
    pub staking_instance: Account<'info, StakingInstance>,

    #[account(
        mut,
        seeds = [
            crate::USER_SEED.as_ref(),
            staking_instance.key().as_ref(),
            authority.key().as_ref()
        ],
        bump = _staking_user_bump,
    )]
    pub user_instance: Account<'info, User>,

    #[account(
        constraint = allowed_collection_address.key()
        == staking_instance.allowed_collection_address,
    )]
    /// CHECK
    pub allowed_collection_address: AccountInfo<'info>,

    #[account(
        constraint =
        token_program.key() == crate::TOKEN_PROGRAM_BYTES.parse::<Pubkey>().unwrap(),
    )]
    /// CHECK
    pub token_program: AccountInfo<'info>,

    #[account(
        constraint =
        nft_program_id.key() ==
        crate::NFT_TOKEN_PROGRAM_BYTES.parse::<Pubkey>().unwrap(),
    )]
    /// CHECK
    pub nft_program_id: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    /// CHECK
    pub rent: AccountInfo<'info>,

    pub time: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
#[instruction(
    staking_instance_bump: u8,
    _staking_user_bump: u8,
)]
pub struct CancelStaking<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        constraint = reward_token_mint
            .mint_authority
            .unwrap()
            .eq(&staking_instance.key())
    )]
    pub reward_token_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub nft_token_mint: Box<Account<'info, Mint>>,

    #[account(
        constraint = nft_token_metadata.owner == &nft_program_id.key(),
    )]
    /// CHECK
    pub nft_token_metadata: AccountInfo<'info>,

    #[account(
        mut,
        constraint = nft_token_authority_wallet
            .clone().into_inner().deref().owner == authority.key(),
        constraint = nft_token_authority_wallet
            .clone().into_inner().deref().mint == nft_token_mint.key(),
    )]
    pub nft_token_authority_wallet: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = nft_token_program_wallet
            .clone().into_inner().deref().owner == staking_instance.key(),
        constraint = nft_token_program_wallet
            .clone().into_inner().deref().mint == nft_token_mint.key(),
    )]
    pub nft_token_program_wallet: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [crate::STAKING_SEED.as_ref(),staking_instance.authority.as_ref()],
        bump = staking_instance_bump,
    )]
    pub staking_instance: Account<'info, StakingInstance>,

    #[account(
        mut,
        seeds = [
            crate::USER_SEED.as_ref(),
            staking_instance.key().as_ref(),
            authority.key().as_ref()
        ],
        bump = _staking_user_bump,
    )]
    pub user_instance: Account<'info, User>,

    #[account(
        constraint = allowed_collection_address.key()
        == staking_instance.allowed_collection_address,
    )]
    /// CHECK
    pub allowed_collection_address: AccountInfo<'info>,

    #[account(
        constraint =
        token_program.key() == crate::TOKEN_PROGRAM_BYTES.parse::<Pubkey>().unwrap(),
    )]
    /// CHECK
    pub token_program: AccountInfo<'info>,

    #[account(
        constraint =
        nft_program_id.key() ==
        crate::NFT_TOKEN_PROGRAM_BYTES.parse::<Pubkey>().unwrap(),
    )]
    /// CHECK
    pub nft_program_id: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    /// CHECK
    pub rent: AccountInfo<'info>,

    pub time: Sysvar<'info,Clock>,
}

#[derive(Accounts)]
#[instruction(
    amount: u64,
    staking_instance_bump: u8,
    _staking_user_bump: u8,
)]
pub struct ClaimRewards<'info> {
    #[account(signer)]
    /// CHECK
    pub authority: AccountInfo<'info>,

    #[account(
        mut,
        constraint = reward_token_mint
            .mint_authority
            .unwrap()
            .eq(&staking_instance.key())
    )]
    pub reward_token_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = reward_token_mint,
        associated_token::authority = authority,
    )]
    pub reward_token_authority_wallet: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [crate::STAKING_SEED.as_ref(),staking_instance.authority.as_ref()],
        bump = staking_instance_bump,
    )]
    pub staking_instance: Box<Account<'info, StakingInstance>>,

    #[account(
        mut,
        seeds = [
            crate::USER_SEED.as_ref(),
            staking_instance.key().as_ref(),
            authority.key().as_ref()
        ],
        bump = _staking_user_bump,
    )]
    pub user_instance: Box<Account<'info, User>>,

    #[account(
        constraint =
        token_program.key() == crate::TOKEN_PROGRAM_BYTES.parse::<Pubkey>().unwrap(),
    )]
    /// CHECK
    pub token_program: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    /// CHECK
    pub rent: AccountInfo<'info>,

    pub time: Sysvar<'info,Clock>,

}