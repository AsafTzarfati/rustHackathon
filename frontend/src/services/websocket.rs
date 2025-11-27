use leptos::*;
use gloo_net::websocket::{futures::WebSocket, Message};
use futures::{StreamExt, SinkExt};
use shared::MessageWrapper;
use wasm_bindgen_futures::spawn_local;

#[derive(Clone)]
pub struct WebSocketService {
    pub sender: Option<futures::channel::mpsc::Sender<Vec<u8>>>,
}

impl WebSocketService {
    pub fn new(on_message: impl Fn(MessageWrapper) + 'static + Clone) -> Self {
        let (sender, receiver) = futures::channel::mpsc::channel::<Vec<u8>>(100);
        let receiver = std::rc::Rc::new(std::cell::RefCell::new(Some(receiver)));
        
        create_effect(move |_| {
            let on_message = on_message.clone();
            let receiver = receiver.clone();
            let location = web_sys::window().unwrap().location();
            let protocol = if location.protocol().unwrap() == "https:" { "wss" } else { "ws" };
            let host = location.host().unwrap();
            let ws_url = format!("{}://{}/ws", protocol, host);

            spawn_local(async move {
                match WebSocket::open(&ws_url) {
                    Ok(ws) => {
                        let (mut write, mut read) = ws.split();
                        
                        // Handle outgoing messages
                        if let Some(mut rx) = receiver.borrow_mut().take() {
                            spawn_local(async move {
                                while let Some(bytes) = rx.next().await {
                                    let _ = write.send(Message::Bytes(bytes)).await;
                                }
                            });
                        }

                        // Handle incoming messages
                        while let Some(msg) = read.next().await {
                            if let Ok(Message::Bytes(bytes)) = msg {
                                if let Ok(wrapper) = MessageWrapper::from_bytes(&bytes) {
                                    on_message(wrapper);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        leptos::logging::error!("Failed to connect to WebSocket: {:?}", e);
                    }
                }
            });
        });

        Self {
            sender: Some(sender),
        }
    }

    pub fn send(&self, msg: MessageWrapper) {
        if let Some(mut sender) = self.sender.clone() {
            spawn_local(async move {
                if let Ok(bytes) = msg.to_bytes() {
                    let _ = sender.try_send(bytes);
                }
            });
        }
    }
}
