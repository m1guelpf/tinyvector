use aide::openapi::{self, OpenApi};
use anyhow::Result;
use axum::Extension;
use std::{env, net::SocketAddr, thread, time};

use crate::{db, routes, shutdown};

#[allow(clippy::redundant_pub_crate)]
pub(crate) async fn start() -> Result<()> {
	let mut openapi = OpenApi {
		info: openapi::Info {
			title: "Tinyvector".to_string(),
			version: env!("CARGO_PKG_VERSION").to_string(),
			..openapi::Info::default()
		},
		..OpenApi::default()
	};

	let db = db::from_store()?;
	let router = routes::handler()
		.finish_api(&mut openapi)
		.layer(Extension(openapi))
		.layer(db.extension());

	let addr = SocketAddr::from((
		[0, 0, 0, 0],
		env::var("PORT").map_or(Ok(8000), |p| p.parse())?,
	));
	tracing::info!("Starting server on {addr}...");
	let server_fut = axum_server::bind(addr)
		.handle(shutdown::handle())
		.serve(router.into_make_service());

	let shutdown_signal_fut = shutdown::wait_for_signal();
	tokio::select! {
		() = shutdown_signal_fut => shutdown::trigger(),
		res = server_fut => res?,
	}

	tracing::info!("Stopping server...");
	thread::sleep(time::Duration::from_secs(1));
	Ok(())
}
