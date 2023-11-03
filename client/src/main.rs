use yew::prelude::*;

mod client_grpc;
mod message_fetcher;
mod message_grpc;

#[function_component(App)]
fn app_body() -> Html {
    html! {
        <message_fetcher::MessageFetcher />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
