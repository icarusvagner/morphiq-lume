use std::net::SocketAddr;

use tonic::{
	Request,
	Response,
	Status,
	service::LayerExt,
	transport::Server,
};
use tracing_subscriber::{
	EnvFilter,
	fmt,
};

mod server;
mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let tracing_format = fmt::format()
		.with_target(false)
		.with_thread_ids(false)
		.with_thread_names(true)
		.with_level(true)
		.compact();
	tracing_subscriber::fmt()
		.with_env_filter(EnvFilter::from_default_env())
		.event_format(tracing_format)
		.init();

	let addr: SocketAddr = "[::1]:50051".parse()?;

	server::start_server(addr).await?;

	Ok(())
}
