#[cfg(feature = "test-bpf")]
mod common;

use crate::common::Env;
use pixel_platform::id;
use pixel_platform::instruction::PlatformInstruction;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Signer;
use solana_sdk::transaction::Transaction;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_generate_vault() {
    let env = Env::new().await;

    let program_id = id();

    let instruction = PlatformInstruction::generate_vault(env.admin.pubkey(), program_id);

    let mut tx = Transaction::new_with_payer(&[instruction], Some(&env.admin.pubkey()));

    tx.sign(&vec![&env.admin], env.recent_blockhash);

    env.client
        .send_transaction(&tx)
        .expect("Transaction failed.");

    let (vault_pda, _) = Pubkey::find_program_address(&["vault".as_bytes()], &program_id);

    assert_eq!(
        vault_pda,
        "63HeLroEXLDnJWcTTmjkkUFg1dqTFiRx8zd3aCWNhprF"
            .parse::<Pubkey>()
            .unwrap()
    );
}
