use std::sync::Arc;

use serde_json::{Map, Value, json};

use crate::{
    Endpoint,
    http::{ApiClient, errors::ApiError},
    types::{DatabaseMetrics, DatabaseStatus},
};

pub struct DatabaseResource {
    pub client: Arc<ApiClient>,
    pub id: String,
}

impl DatabaseResource {
    pub fn new(http: Arc<ApiClient>, id: &str) -> Self {
        Self {
            client: http,
            id: id.to_string(),
        }
    }

    pub async fn start(&self) -> Result<bool, ApiError> {
        self.client
            .request_endpoint::<()>(Endpoint::start_database(&self.id))
            .await?
            .into_bool_result()
    }

    pub async fn stop(&self) -> Result<bool, ApiError> {
        self.client
            .request_endpoint::<()>(Endpoint::stop_database(&self.id))
            .await?
            .into_bool_result()
    }

    pub async fn status(&self) -> Result<DatabaseStatus, ApiError> {
        self.client
            .request_endpoint(Endpoint::database_status(&self.id))
            .await?
            .into_result_t()
    }

    pub async fn metrics(&self) -> Result<Vec<DatabaseMetrics>, ApiError> {
        self.client
            .request_endpoint(Endpoint::database_metrics(&self.id))
            .await?
            .into_result_t()
    }

    pub async fn edit(
        &self,
        name: Option<&str>,
        ram: Option<u32>,
    ) -> Result<bool, ApiError> {
        if name.is_none() && ram.is_none() {
            return Ok(false);
        }
        let mut map = Map::new();
        if name.is_some() {
            map.insert(
                "name".to_string(),
                Value::String(name.unwrap().to_string()),
            );
        }
        if ram.is_some() {
            map.insert("ram".to_string(), Value::Number(ram.unwrap().into()));
        }
        let payload = Value::Object(map);
        let endpoint = Endpoint::edit_database(&self.id);
        let request = endpoint
            .request_builder(&self.client.http_client)
            .json(&payload)
            .build()?;
        self.client
            .execute_request::<()>(request)
            .await?
            .into_bool_result()
    }
}
