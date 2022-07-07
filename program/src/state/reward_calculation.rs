use crate::consts::{MAX_PAYOUT_PER_NFT, REWARD_PERIOD, SECONDS_IN_THE_DAY};

pub fn calculate_reward(
    clock_timestamp: u64,
    stake_timestamp: u64,
    harvested: u64,
    withdrawn: u64,
) -> u64 {
    let periods = (clock_timestamp - stake_timestamp) / SECONDS_IN_THE_DAY;

    let mut reward: f64 = 0.0;
    for day in 0..periods {
        if day >= REWARD_PERIOD - 1 {
            break;
        }
        reward += 0.75;
    }

    reward -= withdrawn as f64;

    if reward > (MAX_PAYOUT_PER_NFT - harvested) as f64 {
        return MAX_PAYOUT_PER_NFT - harvested;
    }

    reward as u64
}
