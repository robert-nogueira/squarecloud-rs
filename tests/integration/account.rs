use squarecloud::ApiClient;

#[tokio::test]
async fn me_returns_account_info() {
    crate::setup();
    let client = ApiClient::new();
    let me = client.me().await.expect("me() should return account info");

    assert!(!me.user.id.is_empty(), "id should not be empty");
    assert!(!me.user.name.is_empty(), "name should not be empty");
    assert!(me.user.email.contains('@'), "email should be valid");
    assert!(
        !me.user.plan.name.is_empty(),
        "plan name should not be empty"
    );
    assert!(
        me.user.plan.memory.limit > 0,
        "plan memory limit should be positive"
    );
    assert!(
        me.user.plan.memory.limit >= me.user.plan.memory.used,
        "used memory should not exceed limit"
    );
}
