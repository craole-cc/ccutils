use crate::{config, config_init, WebConfig};
use anyhow::Result;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use tword_auth::tls;

pub async fn init() -> Result<Arc<WebConfig>> {
	let cfg = config_init::<WebConfig>().await?;
	Ok(cfg)
}

pub async fn reload() -> Result<Arc<WebConfig>> {
	let cfg = config::reload::<WebConfig>().await?;
	Ok(cfg)
}

// pub async fn serve() -> Result<(TlsAcceptor, TcpListener)> {
// 	let service = init().await?;
// 	let service_name = "Web";
// 	// let socket_address = service.socket_address;
// 	let certificate = &service.tls_certificate;
// 	let private_key = &service.tls_private_key;
// 	// let port = service.port;

// 	let (acceptor, listener) = tls::serve(
// 		service_name,
// 		// socket_address,
// 		certificate,
// 		private_key,
// 	)
// 	.await?;

// 	Ok((acceptor, listener))
// }
