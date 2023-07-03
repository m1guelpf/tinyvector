#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use anyhow::Result;
use tracing_subscriber::{
	prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer,
};

mod routes;
mod server;
mod shutdown;

#[tokio::main]
async fn main() -> Result<()> {
	tracing_subscriber::registry()
		.with(tracing_subscriber::fmt::layer().with_filter(
			EnvFilter::try_from_default_env().unwrap_or_else(|_| "tinyvector=info".into()),
		))
		.init();

	server::start().await
}
