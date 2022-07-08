use crate::consts::{MAX_PAYOUT_PER_NFT, PAYOUT_PER_DAY, SECONDS_IN_THE_DAY};

pub fn calculate_reward(
    clock_timestamp: u64,
    stake_timestamp: u64,
    harvested: u64,
    withdrawn: u64,
) -> u64 {
    let periods = (clock_timestamp - stake_timestamp) / SECONDS_IN_THE_DAY;

    let mut reward = match periods {
        0..=1 => 0,
        2..=180 => {
            let mut reward = 0;
            for day in 2..=periods {
                reward += PAYOUT_PER_DAY * (day - 1);
            }
            reward
        }
        _ => MAX_PAYOUT_PER_NFT,
    };

    reward -= withdrawn;

    if reward >= (MAX_PAYOUT_PER_NFT - harvested) {
        return MAX_PAYOUT_PER_NFT - harvested;
    }

    reward
}
