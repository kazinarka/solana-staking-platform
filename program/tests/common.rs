#[cfg(feature = "test-bpf")]
use solana_client::rpc_client::RpcClient;
use solana_program::hash::Hash;
use solana_program::pubkey::Pubkey;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::Keypair;

#[allow(dead_code)]
pub struct Env {
    pub client: RpcClient,
    pub admin: Keypair,
    pub user: Keypair,
    pub creator: Pubkey,
    pub nft: Pubkey,
    pub metadata: Pubkey,
    pub metadata_data: Vec<u8>,
    pub reward_mint: Pubkey,
    pub recent_blockhash: Hash,
}

impl Env {
    #[allow(dead_code)]
    pub async fn new() -> Self {
        let client = RpcClient::new_with_commitment(
            "https://api.devnet.solana.com".to_string(),
            CommitmentConfig::confirmed(),
        );

        let admin = Keypair::from_bytes(&[
            89, 61, 139, 211, 93, 133, 1, 223, 48, 48, 225, 41, 130, 190, 150, 113, 99, 31, 182,
            234, 148, 252, 9, 237, 231, 248, 4, 122, 35, 46, 142, 49, 194, 67, 195, 223, 140, 248,
            45, 171, 238, 145, 41, 230, 118, 18, 83, 60, 130, 228, 142, 74, 151, 98, 167, 191, 113,
            20, 185, 14, 156, 242, 207, 121,
        ])
        .unwrap();

        let user = Keypair::from_bytes(&[
            10, 26, 54, 75, 16, 91, 173, 140, 255, 47, 59, 217, 202, 106, 93, 94, 69, 39, 231, 189,
            210, 199, 34, 100, 213, 177, 7, 54, 185, 15, 31, 36, 151, 11, 113, 206, 82, 209, 189,
            100, 157, 91, 69, 231, 99, 16, 224, 121, 154, 35, 128, 39, 107, 144, 38, 39, 107, 155,
            223, 203, 122, 242, 141, 78,
        ])
        .unwrap();

        let creator = "BW9mMN9MsWBuk7Go3cPKzSjK3nTx1dzvRuvqB7YN1rE6"
            .parse::<Pubkey>()
            .unwrap();

        let nft = "8jYoxcCxnjmT4hGWKbr2R2iEmhzrzmQngDn4pwKFH5sY"
            .parse::<Pubkey>()
            .unwrap();

        let reward_mint = "5SF89AifEF7g6QhtKXwX6GjJXCxJAteBjPBn3XTWo4vP"
            .parse::<Pubkey>()
            .unwrap();

        let (metadata, _) = Pubkey::find_program_address(
            &[
                "metadata".as_bytes(),
                &spl_token_metadata::ID.to_bytes(),
                &nft.to_bytes(),
            ],
            &spl_token_metadata::ID,
        );

        let metadata_data = client.get_account_data(&metadata).unwrap();

        let recent_blockhash = client.get_latest_blockhash().expect("Can't get blockhash");

        Self {
            client,
            admin,
            user,
            creator,
            nft,
            metadata,
            metadata_data,
            reward_mint,
            recent_blockhash,
        }
    }
}
