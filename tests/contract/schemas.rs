use squarecloud_rs::types::{AppInfo, Deploy, DnsRecord, FileInfo, Plan, Snapshot, UserInfo};

use crate::validation::generate_json_variants_from_schema;

fn check_schema<T: serde::de::DeserializeOwned>(
    schema_name: &str,
    spec: &serde_json::Value,
) -> Vec<String> {
    let schemas = &spec["components"]["schemas"];
    let variants =
        generate_json_variants_from_schema(&schemas[schema_name], schemas);
    variants
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
