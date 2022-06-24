use crate::Timestamp;

pub const NFT_AMOUNT: u64 = 3500;

pub const REWARD_PERIOD: u64 = 180;

pub const SECONDS_IN_THE_DAY: Timestamp = 24 * 60 * 60;

pub const SECONDS_IN_THE_REWARD_PERIOD: Timestamp = SECONDS_IN_THE_DAY * REWARD_PERIOD;

pub const MAX_PAYOUT_PER_DAY: f64 = 13.3;

pub const MAX_PAYOUT_PER_NFT: u64 = 1198;

pub const MAX_TOTAL_DAILY_EMISSION_PER_DAY: f64 = MAX_PAYOUT_PER_DAY * NFT_AMOUNT as f64;

pub const TOTAL_EMISSIONS: u64 = MAX_PAYOUT_PER_NFT * NFT_AMOUNT;

pub const M: f64 = 130.171446306642;

pub const X: u64 = 2;

pub const MX: f64 = M * X as f64;

pub const VAULT: &[u8] = "vault".as_bytes();

pub const WHITELIST: &[u8] = "whitelist".as_bytes();