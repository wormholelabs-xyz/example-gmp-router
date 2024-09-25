mod common;

use common::setup::{get_account, setup};
use router::state::Config;

#[tokio::test]
async fn test_initialize() {
    // Set up the test environment
    let (mut context, owner_pubkey, config_pda) = setup().await;

    // Verify the state after initialization
    let config: Config = get_account(&mut context.banks_client, config_pda).await;

    assert_eq!(config.owner, owner_pubkey);
    assert_eq!(config.paused, false);
    assert_eq!(config.next_integrator_id, 0);
}
