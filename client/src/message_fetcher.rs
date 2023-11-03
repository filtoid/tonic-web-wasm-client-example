use yew::prelude::*;
use crate::message_grpc;

pub enum Msg {
    FetchUpdateClicked,
    MessageReceived(String),
    ErrorReceived(String),
}

pub struct MessageControl {
    message: String,
    status: String
}

impl Component for MessageControl {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            message: "".to_string(),
            status: "".to_string()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchUpdateClicked => {
                ctx.link().send_future(async {
                    match message_grpc::perform_get_message_grpc().await {
                        Ok(message) => Msg::MessageReceived(message),
                        Err(err) => Msg::ErrorReceived(err)
                    }
                });
                false
            },
            Msg::MessageReceived(message) => {
                self.message = message;
                true
            },
            Msg::ErrorReceived(error_string) => {
                self.status = error_string;
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let get_new_message = ctx.link().callback(move |_:MouseEvent|{
            Msg::FetchUpdateClicked
        });

        html! {
            <div>
                <h1>{"Status - "}{self.status.clone()}</h1>
                <h1>{"Message - "}{self.message.clone()}</h1>
                <div class={"button"} onclick={get_new_message}>
                    { "Submit" }
                </div>
            </div>
        }
    }

}

#[function_component(MessageFetcher)]
pub fn message_component() -> Html {
 
    html! {
        <MessageControl />
    }
}