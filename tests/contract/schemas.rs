use squarecloud::types::{
    Analytics, AppInfo, AppSummary, Database, DatabaseInfo, DatabaseSummary,
    Deploy, DnsRecord, FileInfo, NetworkErrors, Plan, RestoreStatus,
    RuntimeStats, RuntimeStatsListItem, Snapshot, UserInfo, WorkspaceApp,
    WorkspaceInfo, WorkspaceMember,
};

use crate::validation::generate_json_variants_from_schema;

fn check_schema<T: serde::de::DeserializeOwned>(
    schema_name: &str,
    spec: &serde_json::Value,
) -> Vec<String> {
    let schemas = &spec["components"]["schemas"];
    check_schema_value::<T>(&schemas[schema_name], schemas)
}

/// Like [`check_schema`], for a schema that is not a named component.
fn check_schema_value<T: serde::de::DeserializeOwned>(
    schema: &serde_json::Value,
    schemas: &serde_json::Value,
) -> Vec<String> {
    generate_json_variants_from_schema(schema, schemas)
        .iter()
        .filter_map(|v| {
            serde_json::from_value::<T>(v.clone())
                .err()
                .map(|e| format!("{e}\n    json: {v}"))
        })
        .collect()
}

macro_rules! assert_schema {
    ($spec:expr, $schema:literal, $type:ty) => {
        let failures = check_schema::<$type>($schema, $spec);
        if !failures.is_empty() {
            panic!(
                "{} failure(s) deserializing {} as {}:\n  {}",
                failures.len(),
                $schema,
                stringify!($type),
                failures.join("\n  ")
            );
        }
    };
}

#[tokio::test]
async fn app_schema_deserializes_as_app_info() {
    let spec = crate::fetch_full_spec().await;
    assert_schema!(&spec, "App", AppInfo);
}

#[tokio::test]
async fn snapshot_schema_deserializes_as_snapshot() {
    let spec = crate::fetch_full_spec().await;
    assert_schema!(&spec, "Snapshot", Snapshot);
}

#[tokio::test]
async fn dns_record_schema_deserializes_as_dns_record() {
    let spec = crate::fetch_full_spec().await;
    assert_schema!(&spec, "DNSRecord", DnsRecord);
}

#[tokio::test]
async fn plan_schema_deserializes_as_plan() {
    let spec = crate::fetch_full_spec().await;
    assert_schema!(&spec, "Plan", Plan);
}

#[tokio::test]
async fn user_schema_deserializes_as_user_info() {
    let spec = crate::fetch_full_spec().await;
    assert_schema!(&spec, "User", UserInfo);
}

#[tokio::test]
async fn file_entry_schema_deserializes_as_file_info() {
    let spec = crate::fetch_full_spec().await;
    assert_schema!(&spec, "FileEntry", FileInfo);
}

#[tokio::test]
async fn deploy_event_schema_deserializes_as_deploy() {
    let spec = crate::fetch_full_spec().await;
    assert_schema!(&spec, "DeployEvent", Deploy);
}

#[tokio::test]
async fn database_created_schema_deserializes_as_database() {
    let spec = crate::fetch_full_spec().await;
    assert_schema!(&spec, "DatabaseCreated", Database);
}

#[tokio::test]
async fn database_schema_deserializes_as_database_info() {
    let spec = crate::fetch_full_spec().await;
    assert_schema!(&spec, "Database", DatabaseInfo);
}

#[tokio::test]
async fn app_summary_schema_deserializes_as_app_summary() {
    let spec = crate::fetch_full_spec().await;
    assert_schema!(&spec, "AppSummary", AppSummary);
}

#[tokio::test]
async fn workspace_member_schema_deserializes_as_workspace_member() {
    let spec = crate::fetch_full_spec().await;
    assert_schema!(&spec, "WorkspaceMember", WorkspaceMember);
}

#[tokio::test]
async fn workspace_app_schema_deserializes_as_workspace_app() {
    let spec = crate::fetch_full_spec().await;
    assert_schema!(&spec, "WorkspaceApp", WorkspaceApp);
}

#[tokio::test]
async fn workspace_schema_deserializes_as_workspace_info() {
    let spec = crate::fetch_full_spec().await;
    assert_schema!(&spec, "Workspace", WorkspaceInfo);
}

#[tokio::test]
async fn database_summary_schema_deserializes_as_database_summary() {
    let spec = crate::fetch_full_spec().await;
    assert_schema!(&spec, "DatabaseSummary", DatabaseSummary);
}

#[tokio::test]
async fn network_analytics_schema_deserializes_as_analytics() {
    let spec = crate::fetch_full_spec().await;
    assert_schema!(&spec, "NetworkAnalytics", Analytics);
}

#[tokio::test]
async fn network_errors_schema_deserializes_as_network_errors() {
    let spec = crate::fetch_full_spec().await;
    assert_schema!(&spec, "NetworkErrors", NetworkErrors);
}

#[tokio::test]
async fn runtime_stats_schema_deserializes_as_runtime_stats() {
    let spec = crate::fetch_full_spec().await;
    assert_schema!(&spec, "RuntimeStats", RuntimeStats);
}

#[tokio::test]
async fn runtime_stats_list_item_schema_deserializes_as_runtime_stats_list_item()
 {
    let spec = crate::fetch_full_spec().await;
    assert_schema!(&spec, "RuntimeStatsListItem", RuntimeStatsListItem);
}

/// The restore state has no component of its own: it is the `response`
/// field of the `SnapshotRestoreStateResult` envelope.
#[tokio::test]
async fn snapshot_restore_state_deserializes_as_restore_status() {
    let spec = crate::fetch_full_spec().await;
    let schemas = &spec["components"]["schemas"];
    let state =
        &schemas["SnapshotRestoreStateResult"]["properties"]["response"];
    let failures = check_schema_value::<RestoreStatus>(state, schemas);
    assert!(
        failures.is_empty(),
        "{} failure(s) deserializing the restore state as RestoreStatus:\n  {}",
        failures.len(),
        failures.join("\n  ")
    );
}
