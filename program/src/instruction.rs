use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::borsh::try_from_slice_unchecked;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;
use solana_program::system_program;

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum PlatformInstruction {
    GenerateVault,
    AddToWhitelist,
    Stake,
    Unstake,
    Claim,
}

impl PlatformInstruction {
    pub fn generate_vault(wallet_pubkey: Pubkey, program_id: Pubkey) -> Instruction {
        let (vault_pda, _) = Pubkey::find_program_address(&["vault".as_bytes()], &program_id);

        Instruction::new_with_borsh(
            program_id,
            &PlatformInstruction::GenerateVault,
            vec![
                AccountMeta::new(wallet_pubkey, true),
                AccountMeta::new(system_program::id(), false),
                AccountMeta::new(vault_pda, false),
                AccountMeta::new_readonly(
                    "SysvarRent111111111111111111111111111111111"
                        .parse::<Pubkey>()
                        .unwrap(),
                    false,
                ),
            ],
        )
    }

    pub fn add_to_whitelist(
        wallet_pubkey: Pubkey,
        creator: Pubkey,
        program_id: Pubkey,
    ) -> Instruction {
        let (wl_address, _) = Pubkey::find_program_address(
            &["whitelist".as_bytes(), &creator.to_bytes()],
            &program_id,
        );

        Instruction::new_with_borsh(
            program_id,
            &PlatformInstruction::AddToWhitelist,
            vec![
                AccountMeta::new(wallet_pubkey, true),
                AccountMeta::new(creator, false),
                AccountMeta::new(wl_address, false),
                AccountMeta::new(system_program::id(), false),
                AccountMeta::new_readonly(
                    "SysvarRent111111111111111111111111111111111"
                        .parse::<Pubkey>()
                        .unwrap(),
                    false,
                ),
            ],
        )
    }

    pub fn stake(
        wallet_pubkey: Pubkey,
        nft: Pubkey,
        program_id: Pubkey,
        metadata: Pubkey,
        metadata_data: Vec<u8>,
    ) -> Instruction {
        let (vault, _vault_bump) = Pubkey::find_program_address(&["vault".as_bytes()], &program_id);

        let source =
            spl_associated_token_account::get_associated_token_address(&wallet_pubkey, &nft);

        let destination = spl_associated_token_account::get_associated_token_address(&vault, &nft);

        let (stake_data, _) = Pubkey::find_program_address(&[&nft.to_bytes()], &program_id);

        let metadata_data_struct: spl_token_metadata::state::Metadata =
            try_from_slice_unchecked(&metadata_data[..]).unwrap();

        let creator = metadata_data_struct
            .data
            .creators
            .unwrap()
            .first()
            .unwrap()
            .address;

        let (wl_data_address, _wl_data_address_bump) = Pubkey::find_program_address(
            &["whitelist".as_bytes(), &creator.to_bytes()],
            &program_id,
        );

        Instruction::new_with_borsh(
            program_id,
            &PlatformInstruction::Stake,
            vec![
                AccountMeta::new(wallet_pubkey, true),
                AccountMeta::new_readonly(nft, false),
                AccountMeta::new_readonly(metadata, false),
                AccountMeta::new_readonly(vault, false),
                AccountMeta::new(source, false),
                AccountMeta::new(destination, false),
                AccountMeta::new_readonly(spl_token::id(), false),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(
                    "SysvarRent111111111111111111111111111111111"
                        .parse::<Pubkey>()
                        .unwrap(),
                    false,
                ),
                AccountMeta::new_readonly(
                    "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
                        .parse::<Pubkey>()
                        .unwrap(),
                    false,
                ),
                AccountMeta::new(stake_data, false),
                AccountMeta::new(wl_data_address, false),
            ],
        )
    }

    pub fn unstake(
        wallet_pubkey: Pubkey,
        nft: Pubkey,
        program_id: Pubkey,
        reward_mint: Pubkey,
        metadata: Pubkey,
        metadata_data: Vec<u8>,
    ) -> Instruction {
        let (vault, _vault_bump) = Pubkey::find_program_address(&["vault".as_bytes()], &program_id);

        let destination =
            spl_associated_token_account::get_associated_token_address(&wallet_pubkey, &nft);

        let source = spl_associated_token_account::get_associated_token_address(&vault, &nft);

        let reward_destination = spl_associated_token_account::get_associated_token_address(
            &wallet_pubkey,
            &reward_mint,
        );

        let reward_source =
            spl_associated_token_account::get_associated_token_address(&vault, &reward_mint);

        let (stake_data, _) = Pubkey::find_program_address(&[&nft.to_bytes()], &program_id);

        let metadata_data_struct: spl_token_metadata::state::Metadata =
            try_from_slice_unchecked(&metadata_data[..]).unwrap();

        let creator = metadata_data_struct
            .data
            .creators
            .unwrap()
            .first()
            .unwrap()
            .address;

        let (wl_data_address, _wl_data_address_bump) = Pubkey::find_program_address(
            &["whitelist".as_bytes(), &creator.to_bytes()],
            &program_id,
        );

        Instruction::new_with_borsh(
            program_id,
            &PlatformInstruction::Unstake,
            vec![
                AccountMeta::new(wallet_pubkey, true),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(nft, false),
                AccountMeta::new_readonly(spl_token::id(), false),
                AccountMeta::new_readonly(
                    "SysvarRent111111111111111111111111111111111"
                        .parse::<Pubkey>()
                        .unwrap(),
                    false,
                ),
                AccountMeta::new_readonly(
                    "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
                        .parse::<Pubkey>()
                        .unwrap(),
                    false,
                ),
                AccountMeta::new(stake_data, false),
                AccountMeta::new_readonly(vault, false),
                AccountMeta::new(reward_destination, false),
                AccountMeta::new(reward_source, false),
                AccountMeta::new(destination, false),
                AccountMeta::new(source, false),
                AccountMeta::new_readonly(metadata, false),
                AccountMeta::new(wl_data_address, false),
                AccountMeta::new_readonly(reward_mint, false),
            ],
        )
    }

    pub fn claim(
        wallet_pubkey: Pubkey,
        nft: Pubkey,
        program_id: Pubkey,
        reward_mint: Pubkey,
        metadata: Pubkey,
        metadata_data: Vec<u8>,
    ) -> Instruction {
        let (vault, _vault_bump) = Pubkey::find_program_address(&["vault".as_bytes()], &program_id);

        let destination =
            spl_associated_token_account::get_associated_token_address(&wallet_pubkey, &nft);

        let source = spl_associated_token_account::get_associated_token_address(&vault, &nft);

        let reward_destination = spl_associated_token_account::get_associated_token_address(
            &wallet_pubkey,
            &reward_mint,
        );

        let reward_source =
            spl_associated_token_account::get_associated_token_address(&vault, &reward_mint);

        let (stake_data, _) = Pubkey::find_program_address(&[&nft.to_bytes()], &program_id);

        let metadata_data_struct: spl_token_metadata::state::Metadata =
            try_from_slice_unchecked(&metadata_data[..]).unwrap();

        let creator = metadata_data_struct
            .data
            .creators
            .unwrap()
            .first()
            .unwrap()
            .address;

        let (wl_data_address, _wl_data_address_bump) = Pubkey::find_program_address(
            &["whitelist".as_bytes(), &creator.to_bytes()],
            &program_id,
        );

        Instruction::new_with_borsh(
            program_id,
            &PlatformInstruction::Claim,
            vec![
                AccountMeta::new(wallet_pubkey, true),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(nft, false),
                AccountMeta::new_readonly(spl_token::id(), false),
                AccountMeta::new_readonly(
                    "SysvarRent111111111111111111111111111111111"
                        .parse::<Pubkey>()
                        .unwrap(),
                    false,
                ),
                AccountMeta::new_readonly(
                    "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
                        .parse::<Pubkey>()
                        .unwrap(),
                    false,
                ),
                AccountMeta::new(stake_data, false),
                AccountMeta::new_readonly(vault, false),
                AccountMeta::new(reward_destination, false),
                AccountMeta::new(reward_source, false),
                AccountMeta::new(destination, false),
                AccountMeta::new(source, false),
                AccountMeta::new_readonly(metadata, false),
                AccountMeta::new(wl_data_address, false),
                AccountMeta::new_readonly(reward_mint, false),
            ],
        )
    }
}
