#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::env;

use anyhow::Result;
use dotenvy::dotenv;
use tracing_subscriber::{
	prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer,
};

mod db;
mod errors;
mod routes;
mod server;
mod shutdown;
mod similarity;

#[tokio::main]
async fn main() -> Result<()> {
	dotenv().ok();

	tracing_subscriber::registry()
		.with(tracing_subscriber::fmt::layer().with_filter(
			EnvFilter::try_from_default_env().unwrap_or_else(|_| "tinyvector=info".into()),
		))
		.init();

	assert!(env::var("JWT_SECRET").is_ok(), "JWT_SECRET must be set");

	server::start().await
}
