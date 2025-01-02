use bridge_tests::common::{get_eth_pool_address, setup};
use bridge_tests::siwe::full_login_with_eth_registered;

#[tokio::test]
pub async fn eth_pool_address() {
    let (ic, siwe, bridge) = setup().await;
    let (_, _, _, identity) = full_login_with_eth_registered(&ic, siwe, bridge, None).await;

    let _ = get_eth_pool_address(&ic, bridge, &identity).await;
}
