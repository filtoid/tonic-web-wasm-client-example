use tonic::transport::Server;
use simple_logger::SimpleLogger;
use message_grpc::*;
use message_service_server::MessageServiceServer;
use tower_http::cors::CorsLayer;

mod message_grpc;
mod message_service;

#[derive(Default)]
struct ServiceServer {}

impl ServiceServer {
    pub fn new() -> Self {
        Self{}
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();

    let addr = "127.0.0.1:5051".parse().unwrap();

    let service_server = ServiceServer::default();
    let message_service = MessageServiceServer::new(service_server);

    let layer = CorsLayer::very_permissive();

    Server::builder()
        .accept_http1(true)
        .layer(
            layer
        )
        .add_service(
            tonic_web::enable(message_service)
        )
        .serve(addr)
        .await?;

    Ok(())
}