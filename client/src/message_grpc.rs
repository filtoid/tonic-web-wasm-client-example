use tonic_web_wasm_client::Client;
use gloo::console::log;

use crate::client_grpc::{
    GetMessageRequest,
    GetMessageResponse,
    message_service_client::MessageServiceClient
};

pub async fn perform_get_message_grpc() -> Result<String, String> {
    let base_url = "http://localhost:5051";

    let mut query_client = MessageServiceClient::new(
        Client::new(base_url.to_string())
    );

    // creating a new Request
    let request = tonic::Request::new(
        GetMessageRequest {},
    );
 
    // sending request and waiting for response
    let response: GetMessageResponse = match query_client.get_message(request).await {
        Ok(resp) => resp.into_inner(),
        Err(err) => {
            log!("error received while calling service: ", err.to_string());
            return Err(err.to_string())
        }
    };

    if response.status == "ok" {
        Ok(response.message)
    } else {
        Err(response.error)
    }
}
