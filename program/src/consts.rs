use crate::Timestamp;

pub const NFT_AMOUNT: u64 = 3500;

pub const REWARD_PERIOD: u64 = 180;

pub const SECONDS_IN_THE_DAY: Timestamp = 24 * 60 * 60;

pub const MAX_PAYOUT_PER_DAY: f64 = 13.3;
pub const MAX_PAYOUT_PER_NFT: u64 = 1198;

pub const VAULT: &[u8] = "vault".as_bytes();
pub const WHITELIST: &[u8] = "whitelist".as_bytes();

pub const ADMIN: &str = "E5L2TjtD8nVjNxoEwgizoM4wsdrAtXg52VCnFF4BG2gg";
pub const REWARD_MINT: &str = "7T6Tihm7XaQddHfXoKmzVDFKTS5zxYuDPCkuvu7CQLxi";