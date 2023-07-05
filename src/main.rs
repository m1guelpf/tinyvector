#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use anyhow::Result;
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
	let api_key : &str = option_env!("API_KEY").unwrap_or("");
	if api_key == "" {
		println!("Running in open mode, no API_KEY set");
	}

	tracing_subscriber::registry()
		.with(tracing_subscriber::fmt::layer().with_filter(
			EnvFilter::try_from_default_env().unwrap_or_else(|_| "tinyvector=info".into()),
		))
		.init();

	server::start().await
}
