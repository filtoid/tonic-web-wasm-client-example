use tonic::{Request, Response, Status};
use message_service_server::MessageService;

use crate::ServiceServer;
use crate::message_grpc::*;

#[tonic::async_trait]
impl MessageService for ServiceServer {
    async fn get_message(&self, _req: Request<GetMessageRequest>) -> Result<Response<GetMessageResponse>, Status> {
        
        Ok(Response::new(GetMessageResponse{
            status: "ok".to_string(),
            error: "".to_string(),
            message: "hello world".to_string()
        }))
    }
}
