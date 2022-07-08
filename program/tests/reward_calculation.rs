#[cfg(feature = "test-bpf")]
mod common;

use pixel_platform::consts::{
    MAX_PAYOUT_PER_NFT, PAYOUT_PER_DAY, REWARD_PERIOD, SECONDS_IN_THE_DAY,
};
use pixel_platform::state::reward_calculation::calculate_reward;
use solana_program::msg;

#[tokio::test]
async fn test_reward_calculation() {
    let now = REWARD_PERIOD * SECONDS_IN_THE_DAY * 2;

    let reward = calculate_reward(now, now, 0, 0);
    msg!(
        "edge case - JUST staked (0 seconds in staking pool) => {:?}",
        reward
    );
    assert_eq!(reward, 0);

    let reward = calculate_reward(now, now - SECONDS_IN_THE_DAY + 1, 0, 0);
    msg!("0 day => {:?}", reward);
    assert_eq!(reward, 0);

    let reward = calculate_reward(now, now - SECONDS_IN_THE_DAY, 0, 0);
    msg!("1 day => {:?}", reward);
    assert_eq!(reward, 0);

    let reward = calculate_reward(now, now - SECONDS_IN_THE_DAY - 1, 0, 0);
    msg!("1 day and 1 second => {:?}", reward);
    assert_eq!(reward, 0);

    let mut reward = 0;
    for i in 2..=REWARD_PERIOD {
        let previous_reward = reward;
        reward = calculate_reward(now, now - SECONDS_IN_THE_DAY * i, 0, 0);
        msg!("{:?} day => {:?}", i, reward);
        assert_eq!(reward, PAYOUT_PER_DAY * (i - 1) + previous_reward);
    }

    let reward = calculate_reward(now, now - SECONDS_IN_THE_DAY * (REWARD_PERIOD + 1), 0, 0);
    msg!("181 day => {:?}", reward);
    assert_eq!(reward, MAX_PAYOUT_PER_NFT);

    let reward = calculate_reward(now, 0, 0, 0);
    msg!(
        "edge case - MAX staking time (360 days for this test pool) => {:?}",
        reward
    );
    assert_eq!(reward, MAX_PAYOUT_PER_NFT);

    let reward = calculate_reward(
        now,
        now - SECONDS_IN_THE_DAY * (REWARD_PERIOD + 1),
        MAX_PAYOUT_PER_NFT,
        0,
    );
    msg!("MAX reward harvested => {:?}", reward);
    assert_eq!(reward, 0);

    let reward = calculate_reward(
        now,
        now - SECONDS_IN_THE_DAY * (REWARD_PERIOD + 1),
        MAX_PAYOUT_PER_NFT,
        MAX_PAYOUT_PER_NFT,
    );
    msg!("MAX reward claimed => {:?}", reward);
    assert_eq!(reward, 0);

    let reward = calculate_reward(
        now,
        now - SECONDS_IN_THE_DAY * (REWARD_PERIOD + 1),
        MAX_PAYOUT_PER_NFT / 2,
        MAX_PAYOUT_PER_NFT / 2,
    );
    msg!("50% harvested and claimed => {:?}", reward);
    assert_eq!(reward, MAX_PAYOUT_PER_NFT / 2);

    let reward = calculate_reward(
        now,
        now - SECONDS_IN_THE_DAY * (REWARD_PERIOD + 1),
        MAX_PAYOUT_PER_NFT / 2,
        0,
    );
    msg!("50% harvested => {:?}", reward);
    assert_eq!(reward, MAX_PAYOUT_PER_NFT / 2);

    let reward = calculate_reward(
        now,
        now - SECONDS_IN_THE_DAY * (REWARD_PERIOD + 1),
        MAX_PAYOUT_PER_NFT - 1,
        0,
    );
    msg!("99% harvested => {:?}", reward);
    assert_eq!(reward, 1);

    let reward = calculate_reward(
        now,
        now - SECONDS_IN_THE_DAY * (REWARD_PERIOD + 1),
        MAX_PAYOUT_PER_NFT - PAYOUT_PER_DAY * 180,
        0,
    );
    msg!("99% harvested => {:?}", reward);
    assert_eq!(reward, PAYOUT_PER_DAY * 180);
}
