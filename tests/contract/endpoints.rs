const IMPLEMENTED: &[(&str, &str)] = &[
    ("post", "/v2/apps"),
    ("get", "/v2/apps/domains"),
    ("get", "/v2/apps/status"),
    ("get", "/v2/apps/{appId}"),
    ("delete", "/v2/apps/{appId}"),
    ("post", "/v2/apps/{appId}/commit"),
    ("post", "/v2/apps/{appId}/deploy/webhook"),
    ("get", "/v2/apps/{appId}/deployments"),
    ("get", "/v2/apps/{appId}/deployments/current"),
    ("get", "/v2/apps/{appId}/envs"),
    ("post", "/v2/apps/{appId}/envs"),
    ("put", "/v2/apps/{appId}/envs"),
    ("delete", "/v2/apps/{appId}/envs"),
    ("get", "/v2/apps/{appId}/files"),
    ("get", "/v2/apps/{appId}/files/content"),
    ("put", "/v2/apps/{appId}/files"),
    ("patch", "/v2/apps/{appId}/files"),
    ("delete", "/v2/apps/{appId}/files"),
    ("get", "/v2/apps/{appId}/logs"),
    ("get", "/v2/apps/{appId}/metrics"),
    ("post", "/v2/apps/{appId}/restart"),
    ("get", "/v2/apps/{appId}/realtime"),
    ("get", "/v2/apps/{appId}/snapshots"),
    ("post", "/v2/apps/{appId}/snapshots"),
    ("get", "/v2/apps/{appId}/network/analytics"),
    ("post", "/v2/apps/{appId}/network/custom"),
    ("get", "/v2/apps/{appId}/network/dns"),
    ("get", "/v2/apps/{appId}/network/errors"),
    ("get", "/v2/apps/{appId}/network/logs"),
    ("get", "/v2/apps/{appId}/network/performance"),
    ("post", "/v2/apps/{appId}/network/purge_cache"),
    ("post", "/v2/databases"),
    ("get", "/v2/databases/status"),
    ("get", "/v2/databases/{dbId}"),
    ("delete", "/v2/databases/{dbId}"),
    ("patch", "/v2/databases/{dbId}"),
    ("get", "/v2/databases/{dbId}/status"),
    ("get", "/v2/databases/{dbId}/metrics"),
    ("get", "/v2/databases/{dbId}/snapshots"),
    ("post", "/v2/databases/{dbId}/snapshots"),
    ("get", "/v2/service/status"),
    ("get", "/v2/users/me"),
    ("get", "/v2/users/snapshots"),
    ("post", "/v2/workspaces"),
    ("get", "/v2/workspaces"),
    ("get", "/v2/workspaces/{workspaceId}"),
];

#[tokio::test]
async fn all_exist_in_openapi() {
    let paths = crate::fetch_spec().await;
    let violations: Vec<String> = IMPLEMENTED
        .iter()
        .filter_map(|(method, path)| crate::check(&paths, method, path))
        .collect();

    if !violations.is_empty() {
        panic!(
            "{} contract violation(s):\n  {}",
            violations.len(),
            violations.join("\n  ")
        );
    }
}
