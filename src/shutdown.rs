use axum_server::Handle;
use once_cell::sync::Lazy;
use tokio::{signal, time::Duration};

static HANDLE: Lazy<Handle> = Lazy::new(|| Handle::new());

pub fn handle() -> Handle {
	(*HANDLE).clone()
}

pub fn trigger() {
	HANDLE.graceful_shutdown(Some(Duration::from_secs(1)))
}

pub async fn wait_for_signal() {
	let ctrl_c = async {
		signal::ctrl_c()
			.await
			.expect("failed to install SIGINT handler");
	};

	#[cfg(unix)]
	let terminate = async {
		signal::unix::signal(signal::unix::SignalKind::terminate())
			.expect("failed to install SIGTERM handler")
			.recv()
			.await;
	};

	#[cfg(not(unix))]
	let terminate = std::future::pending::<()>();

	tokio::select! {
		() = ctrl_c => {
			tracing::info!("Received Ctrl+C signal");
		},
		() = terminate => {
			tracing::info!("Received terminate signal");
		},
	}
}
