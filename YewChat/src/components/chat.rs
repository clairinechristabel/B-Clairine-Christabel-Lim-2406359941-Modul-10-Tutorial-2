use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
}
impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                    u
                                )
                                .into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(input.value()),
                        data_array: None,
                    };
                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);

        html! {
            <div class="flex w-screen bg-black font-mono text-green-400 selection:bg-green-900">
                <div class="flex-none w-64 h-screen bg-gray-900 border-r-2 border-green-800 flex flex-col">
                    <div class="text-xl p-4 border-b-2 border-green-800 font-bold tracking-widest text-center shadow-[0_4px_10px_rgba(34,197,94,0.1)]">{"[ CREW ]"}</div>
                    <div class="flex-grow overflow-auto">
                    {
                        self.users.clone().iter().map(|u| {
                            html!{
                                <div class="flex m-3 bg-gray-800 rounded-lg p-2 border border-green-700 shadow-[0_0_5px_rgba(34,197,94,0.3)] items-center">
                                    <div>
                                        <img class="w-10 h-10 rounded-full border border-green-500 bg-black p-1" src={u.avatar.clone().replace("adventurer-neutral", "bottts")} alt="avatar"/>
                                    </div>
                                    <div class="flex-grow px-3">
                                        <div class="flex text-sm justify-between font-bold text-green-300">
                                            <div>{u.name.clone()}</div>
                                        </div>
                                        <div class="text-xs text-green-600">
                                            {"<ONLINE>"}
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                    </div>
                </div>
                <div class="grow h-screen flex flex-col relative">
                    <div class="absolute inset-0 bg-[url('https://www.transparenttextures.com/patterns/stardust.png')] opacity-10 pointer-events-none"></div>
                    <div class="w-full h-14 border-b-2 border-green-800 bg-gray-900 flex justify-between items-center px-6 z-10 shadow-[0_4px_10px_rgba(34,197,94,0.1)]">
                        <div class="text-xl font-bold tracking-widest">{"🌌 GALACTIC COMM LINK"}</div>
                        <div class="text-xs text-green-500 animate-pulse">{"SECURE CONNECTION ESTABLISHED"}</div>
                    </div>
                    <div class="w-full grow overflow-auto p-4 z-10">
                        {
                            self.messages.iter().map(|m| {
                                let user = self.users.iter().find(|u| u.name == m.from).unwrap();
                                html!{
                                    <div class="flex items-start w-3/4 m-4">
                                        <img class="w-10 h-10 rounded-full border border-green-600 bg-black p-1 flex-shrink-0" src={user.avatar.clone().replace("adventurer-neutral", "bottts")} alt="avatar"/>
                                        <div class="p-3 bg-gray-800 ml-3 rounded-tr-xl rounded-br-xl rounded-bl-xl border border-green-800 shadow-[0_0_8px_rgba(34,197,94,0.2)]">
                                            <div class="text-sm font-bold text-green-500 mb-1">
                                                {format!("ID: {}", m.from.clone())}
                                            </div>
                                            <div class="text-sm text-green-300 break-words max-w-full">
                                                if m.message.ends_with(".gif") || m.message.starts_with("http") {
                                                    <img class="mt-2 rounded max-w-full h-auto" src={m.message.clone()}/>
                                                } else {
                                                    {format!("> {}", m.message.clone())}
                                                }
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                    <div class="w-full h-16 flex px-4 items-center bg-gray-900 border-t-2 border-green-800 z-10 shadow-[0_-4px_10px_rgba(34,197,94,0.1)]">
                        <input ref={self.chat_input.clone()} type="text" placeholder="Transmit signal..." class="block w-full py-2 px-4 mx-3 bg-black border border-green-700 rounded-full outline-none focus:shadow-[0_0_12px_rgba(34,197,94,0.6)] text-green-300 placeholder-green-700" name="message" required=true />
                        <button onclick={submit} class="p-3 shadow-[0_0_8px_rgba(34,197,94,0.5)] bg-green-800 hover:bg-green-600 w-12 h-12 rounded-full flex justify-center items-center transition duration-300 flex-shrink-0">
                            <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="fill-green-200 w-6 h-6">
                                <path d="M0 0h24v24H0z" fill="none"></path><path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}
