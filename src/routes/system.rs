use aide::axum::{
	routing::{get, post},
	ApiRouter,
};
use axum_jsonschema::Json;
use schemars::JsonSchema;

use crate::shutdown;

pub fn handler() -> ApiRouter {
	ApiRouter::new()
		.api_route("/", get(root))
		.api_route("/shutdown", post(trigger_shutdown))
}

#[derive(Debug, serde::Serialize, JsonSchema)]
pub struct AppVersion {
	semver: String,
	rev: Option<String>,
	compile_time: String,
}

#[derive(Debug, serde::Serialize, JsonSchema)]
pub struct RootResponse {
	/// Relative URL to Swagger UI
	pub docs_url: String,
	/// Relative URL to OpenAPI specification
	pub openapi_url: String,
	/// Application version
	pub version: AppVersion,
}

#[allow(clippy::unused_async)]
pub async fn root() -> Json<RootResponse> {
	Json(RootResponse {
		docs_url: "/docs".to_string(),
		openapi_url: "/openapi.json".to_string(),
		version: AppVersion {
			semver: env!("CARGO_PKG_VERSION").to_string(),
			compile_time: env!("STATIC_BUILD_DATE").to_string(),
			rev: option_env!("GIT_REV").map(ToString::to_string),
		},
	})
}

#[allow(clippy::unused_async)]
pub async fn trigger_shutdown() -> Json<String> {
	shutdown::trigger();

	Json("Shutting down...".to_string())
}
