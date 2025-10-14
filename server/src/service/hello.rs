use tonic::{
	Request,
	Response,
	Status,
};

use crate::service::hello::greet_hello::{
	SayRequest,
	SayResponse,
	hello_server::{
		self,
		HelloServer,
	},
};

mod greet_hello {
	include!("../../output/hello.rs");
}

#[derive(Default)]
pub struct GreetHello;

#[tonic::async_trait]
impl hello_server::Hello for GreetHello {
	async fn greet(
		&self,
		request: Request<SayRequest>,
	) -> Result<Response<SayResponse>, Status> {
		tracing::info!("{:<12} - {}", "HANDLER", "hello");

		Ok(Response::new(SayResponse {
			message: format!(
				"{} {}",
				request.get_ref().message,
				request.get_ref().name
			),
		}))
	}
}

pub fn new_service() -> HelloServer<GreetHello> {
	HelloServer::new(GreetHello)
}
