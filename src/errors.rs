use aide::OperationOutput;
use axum::{
	http::StatusCode,
	response::{IntoResponse, Response},
	Json,
};
use serde_json::{json, Value};

#[derive(Debug)]
pub struct HTTPError {
	detail: Value,
	status_code: StatusCode,
}

impl HTTPError {
	pub fn new(detail: &str) -> Self {
		Self {
			detail: detail.into(),
			status_code: StatusCode::UNPROCESSABLE_ENTITY,
		}
	}

	pub const fn with_status(mut self, status_code: StatusCode) -> Self {
		self.status_code = status_code;
		self
	}
}

impl IntoResponse for HTTPError {
	fn into_response(self) -> Response {
		(self.status_code, Json(json!({ "error": self.detail }))).into_response()
	}
}

impl OperationOutput for HTTPError {
	type Inner = Self;
}
