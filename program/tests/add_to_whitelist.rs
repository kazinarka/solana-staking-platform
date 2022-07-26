#[cfg(feature = "test-bpf")]
mod common;

use crate::common::Env;
use pixel_platform::id;
use pixel_platform::instruction::PlatformInstruction;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Signer;
use solana_sdk::transaction::Transaction;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_add_to_whitelist() {
    let env = Env::new().await;

    let program_id = id();

    let instruction =
        PlatformInstruction::add_to_whitelist(env.admin.pubkey(), env.creator, program_id);

    let mut tx = Transaction::new_with_payer(&[instruction], Some(&env.admin.pubkey()));

    tx.sign(&vec![&env.admin], env.recent_blockhash);

    env.client
        .send_transaction(&tx)
        .expect("Transaction failed.");

    let (wl_address, _) = Pubkey::find_program_address(
        &["whitelist".as_bytes(), &env.creator.to_bytes()],
        &program_id,
    );

    assert_eq!(
        wl_address,
        "5krAK8y3HiXdZzzqgo9QvsKBv2kAHM6G6FnEw2bLd99n"
            .parse::<Pubkey>()
            .unwrap()
    );
}
