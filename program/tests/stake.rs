#[cfg(feature = "test-bpf")]
mod common;

use crate::common::Env;
use pixel_platform::id;
use pixel_platform::instruction::PlatformInstruction;
use solana_sdk::signature::Signer;
use solana_sdk::transaction::Transaction;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_stake() {
    let env = Env::new().await;

    let program_id = id();

    let instruction = PlatformInstruction::stake(
        env.user.pubkey(),
        env.nft,
        program_id,
        env.metadata,
        env.metadata_data,
    );

    let mut tx = Transaction::new_with_payer(&[instruction], Some(&env.user.pubkey()));

    tx.sign(&vec![&env.user], env.recent_blockhash);

    env.client
        .send_transaction(&tx)
        .expect("Transaction failed.");
}
