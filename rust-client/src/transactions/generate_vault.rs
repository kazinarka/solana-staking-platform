use crate::consts::{PROGRAM_ID, RENT};
use crate::structs::PlatformInstruction;
use clap::ArgMatches;
use solana_client::rpc_client::RpcClient;
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

pub fn generate_vault(matches: &ArgMatches) {
    let program_id = PROGRAM_ID.parse::<Pubkey>().unwrap();

    let url = match matches.value_of("env") {
        Some("dev") => "https://api.devnet.solana.com",
        _ => "https://api.mainnet-beta.solana.com",
    };
    let client = RpcClient::new_with_commitment(url.to_string(), CommitmentConfig::confirmed());

    let wallet_path = matches.value_of("sign").unwrap();
    let wallet_keypair = read_keypair_file(wallet_path).expect("Can't open file-wallet");
    let wallet_pubkey = wallet_keypair.pubkey();

    let (vault_pda, _) = Pubkey::find_program_address(&["vault".as_bytes()], &program_id);

    let instructions = vec![Instruction::new_with_borsh(
        program_id,
        &PlatformInstruction::GenerateVault,
        vec![
            AccountMeta::new(wallet_pubkey, true),
            AccountMeta::new(system_program::id(), false),
            AccountMeta::new(vault_pda, false),
            AccountMeta::new_readonly(RENT.parse::<Pubkey>().unwrap(), false),
        ],
    )];
    let mut tx = Transaction::new_with_payer(&instructions, Some(&wallet_pubkey));
    let recent_blockhash = client.get_latest_blockhash().expect("Can't get blockhash");
    tx.sign(&vec![&wallet_keypair], recent_blockhash);
    let id = client.send_transaction(&tx).expect("Transaction failed.");
    println!("vault account generated: {:?}", vault_pda);
    println!("tx id: {:?}", id);
}
