use tonic_web_wasm_client::{Client,options::FetchOptions};
use gloo::console::log;

// use crate::message_service_client::MessageServiceClient;
use crate::client_grpc::{GetMessageRequest,GetMessageResponse,message_service_client::MessageServiceClient};

pub async fn perform_get_message_grpc() -> Result<String, String> {
    let base_url = "http://localhost:5051";
    let query_opts = FetchOptions{
        mode: Some(tonic_web_wasm_client::options::Mode::NoCors),
        ..Default::default()
    };

    let mut query_client = MessageServiceClient::new(
        Client::new_with_options(
            base_url.to_string(),
            query_opts
        )
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
