#[tokio::test]
async fn all_exist_in_openapi() {
    let paths = crate::fetch_spec().await;
    let violations: Vec<String> =
        inventory::iter::<squarecloud_rs::EndpointSpec>()
            .filter_map(|spec| crate::check(&paths, spec.method, spec.path))
            .collect();

    if !violations.is_empty() {
        panic!(
            "{} contract violation(s):\n  {}",
            violations.len(),
            violations.join("\n  ")
        );
    }
}
