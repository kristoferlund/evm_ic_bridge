use bridge_tests::context::Context;

#[tokio::test]
pub async fn eth_pool_address() {
    let mut context = Context::new();
    let context = context.setup_default().await;
    let _ = context.get_eth_pool_address().await;
    context.teardown_default().await;
}
