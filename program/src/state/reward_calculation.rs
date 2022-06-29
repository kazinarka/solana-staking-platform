use crate::consts::{MAX_PAYOUT_PER_NFT, MX, NFT_AMOUNT, REWARD_PERIOD, SECONDS_IN_THE_DAY};

pub fn calculate_reward(clock_timestamp: u64, stake_timestamp: u64, harvested: u64, withdrawn: u64) -> u64 {
    let periods =
        (clock_timestamp - stake_timestamp) / SECONDS_IN_THE_DAY;

    let mut reward = 0;
    let mut prev_daily_emission: f64 = 1.0;
    for day in 0..periods {
        if day >= REWARD_PERIOD - 1 {
            break;
        }
        prev_daily_emission += MX;
        reward += (prev_daily_emission / NFT_AMOUNT as f64) as u64;
    }

    reward -= withdrawn;

    if reward > MAX_PAYOUT_PER_NFT - harvested {
        reward = MAX_PAYOUT_PER_NFT - harvested;
    }

    reward
}