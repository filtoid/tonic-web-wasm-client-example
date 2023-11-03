use tonic::transport::Server;

use message_grpc::*;
use message_service_server::MessageServiceServer;

mod message_grpc;
mod message_service;

struct ServiceServer {}

impl ServiceServer {
    pub fn new() -> Self {
        Self{}
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::]:5051".parse().unwrap();
    let service = ServiceServer::new();

    Server::builder()
        .accept_http1(true)
        .add_service(MessageServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}