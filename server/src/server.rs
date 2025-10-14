use std::net::SocketAddr;

use tonic::transport::Server;

use crate::service::hello::new_service;

pub async fn start_server(addr: SocketAddr) -> anyhow::Result<()> {
	let hello_service = new_service();

	tracing::info!("Server listen on http://[{}]:{}", addr.ip(), addr.port());

	Server::builder().add_service(hello_service).serve(addr).await?;

	Ok(())
}
