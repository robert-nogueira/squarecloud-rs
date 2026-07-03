use squarecloud_rs::types::AppInfo;

use crate::validation::generate_json_variants_from_schema;

#[tokio::test]
async fn app_schema_deserializes_as_app_info() {
    let spec = crate::fetch_full_spec().await;
    let schemas = &spec["components"]["schemas"];
    let schema = &schemas["App"];

    let variants = generate_json_variants_from_schema(schema, schemas);
    let mut failures = vec![];

    for variant in &variants {
        if let Err(e) = serde_json::from_value::<AppInfo>(variant.clone()) {
            failures.push(format!("{e}\n    json: {variant}"));
        }
    }

    if !failures.is_empty() {
        panic!(
            "{} failure(s) deserializing App schema as AppInfo:\n  {}",
            failures.len(),
            failures.join("\n  ")
        );
    }
}
