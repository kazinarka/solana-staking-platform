use clap::ArgMatches;
use solana_client::rpc_client::RpcClient;
use solana_sdk::borsh::try_from_slice_unchecked;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{read_keypair_file, Signer};
#[allow(unused_imports)]
use solana_sdk::signer::keypair::Keypair;
#[allow(unused_imports)]
use solana_sdk::signer::signers::Signers;
use solana_sdk::system_program;
use solana_sdk::transaction::Transaction;
use crate::consts::{ASSOCIATED_TOKEN, PROGRAM_ID, RENT, REWARD_MINT};
use crate::structs::PlatformInstruction;

pub fn unstake(matches: &ArgMatches) {
    let program_id = PROGRAM_ID.parse::<Pubkey>().unwrap();
    let reward_mint = REWARD_MINT.parse::<Pubkey>().unwrap();

    let url = match matches.value_of("env") {
        Some("dev") => "https://api.devnet.solana.com",
        _ => "https://api.mainnet-beta.solana.com",
    };
    let client = RpcClient::new_with_commitment(url.to_string(), CommitmentConfig::confirmed());

    let wallet_path = matches.value_of("sign").unwrap();
    let wallet_keypair = read_keypair_file(wallet_path).expect("Can't open file-wallet");
    let wallet_pubkey = wallet_keypair.pubkey();

    let nft = matches.value_of("nft").unwrap().parse::<Pubkey>().unwrap();

    let (metadata, _) = Pubkey::find_program_address(
        &[
            "metadata".as_bytes(),
            &spl_token_metadata::ID.to_bytes(),
            &nft.to_bytes(),
        ],
        &spl_token_metadata::ID,
    );

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

    let metadata_data = client.get_account_data(&metadata).unwrap();

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

    let instructions = vec![Instruction::new_with_borsh(
        program_id,
        &PlatformInstruction::Unstake,
        vec![
            AccountMeta::new(wallet_pubkey, true),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(nft, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(RENT.parse::<Pubkey>().unwrap(), false),
            AccountMeta::new_readonly(ASSOCIATED_TOKEN.parse::<Pubkey>().unwrap(), false),
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
    )];

    let mut tx = Transaction::new_with_payer(&instructions, Some(&wallet_pubkey));
    let recent_blockhash = client.get_latest_blockhash().expect("Can't get blockhash");
    tx.sign(&vec![&wallet_keypair], recent_blockhash);
    let id = client.send_transaction(&tx).expect("Transaction failed.");
    println!("tx id: {:?}", id);
}