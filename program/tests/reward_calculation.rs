#[cfg(feature = "test-bpf")]
mod common;

use pixel_platform::state::reward_calculation::calculate_reward;

#[tokio::test]
async fn test_reward_calculation() {
    assert_eq!(calculate_reward(200, 180, 0, 0), 0);
}
