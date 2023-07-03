use aide::axum::{
    routing::{get, post},
    ApiRouter,
};
use axum::Extension;
use axum_jsonschema::Json;
use schemars::JsonSchema;

use crate::shutdown::Agent as Shutdown;

pub fn handler() -> ApiRouter {
    ApiRouter::new()
        .api_route("/", get(root))
        .api_route("/health-check", get(health_check))
        .api_route("/shutdown", post(shutdown))
}

#[derive(Debug, serde::Serialize, JsonSchema)]
pub struct RootResponse {
    /// Relative URL to Swagger UI
    pub docs_url: String,
    /// Relative URL to OpenAPI specification
    pub openapi_url: String,
}

#[allow(clippy::unused_async)]
pub async fn root() -> Json<RootResponse> {
    Json(RootResponse {
        docs_url: "/docs".to_string(),
        openapi_url: "/openapi.json".to_string(),
    })
}

#[allow(clippy::unused_async)]
pub async fn health_check() {}

#[allow(clippy::unused_async)]
pub async fn shutdown(Extension(shutdown): Extension<Shutdown>) -> Json<String> {
    shutdown.start();

    Json("Shutting down...".to_string())
}
