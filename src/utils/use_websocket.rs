use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{MessageEvent, WebSocket};
use yew::hook;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum WebSocketStatus {
    Connecting,
    Open,
    Closing,
    Closed,
    Error(String),
}

#[derive(Clone)]
pub struct UseWebSocketOptions {
    pub reconnect_limit: Option<u32>,
    pub reconnect_interval: Option<u32>,
    pub manual: bool,
}

impl Default for UseWebSocketOptions {
    fn default() -> Self {
        Self {
            reconnect_limit: Some(3),
            reconnect_interval: Some(3000),
            manual: false,
        }
    }
}

#[hook]
pub fn use_websocket(url: String, options: Option<UseWebSocketOptions>) -> (
    WebSocketStatus,
    Rc<WebSocket>,
    impl Fn(),
    impl Fn(String),
    impl Fn(),
) {
    let options = options.unwrap_or_default();
    let ws = yew::use_state(|| None::<WebSocket>);
    let status = yew::use_state(|| WebSocketStatus::Closed);
    let reconnect_times = yew::use_state(|| 0);
    let reconnect_timer = yew::use_state(|| None::<i32>);

    let connect = {
        let url = url.clone();
        let ws = ws.clone();
        let status = status.clone();
        let reconnect_times = reconnect_times.clone();
        
        move || {
            if let Ok(socket) = WebSocket::new(&url) {
                // Message handler
                let onmessage_callback = {
                    let ws = ws.clone();
                    Closure::wrap(Box::new(move |e: MessageEvent| {
                        if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                            // Handle message here
                            log::debug!("Received message: {:?}", txt);
                        }
                    }) as Box<dyn FnMut(MessageEvent)>)
                };

                // Open handler
                let onopen_callback = {
                    let status = status.clone();
                    Closure::wrap(Box::new(move |_| {
                        status.set(WebSocketStatus::Open);
                    }) as Box<dyn FnMut(JsValue)>)
                };

                // Close handler
                let onclose_callback = {
                    let status = status.clone();
                    let reconnect_times = reconnect_times.clone();
                    let url = url.clone();
                    Closure::wrap(Box::new(move |_| {
                        status.set(WebSocketStatus::Closed);
                        
                        if let Some(limit) = options.reconnect_limit {
                            if *reconnect_times < limit {
                                reconnect_times.set(*reconnect_times + 1);
                                // Attempt reconnect
                                if let Ok(new_socket) = WebSocket::new(&url) {
                                    ws.set(Some(new_socket));
                                }
                            }
                        }
                    }) as Box<dyn FnMut(JsValue)>)
                };

                // Error handler
                let onerror_callback = {
                    let status = status.clone();
                    Closure::wrap(Box::new(move |e: JsValue| {
                        status.set(WebSocketStatus::Error(format!("{:?}", e)));
                    }) as Box<dyn FnMut(JsValue)>)
                };

                // Set handlers
                socket.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
                socket.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
                socket.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
                socket.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));

                // Store callbacks to prevent them from being dropped
                onmessage_callback.forget();
                onopen_callback.forget();
                onclose_callback.forget();
                onerror_callback.forget();

                ws.set(Some(socket));
                status.set(WebSocketStatus::Connecting);
            } else {
                status.set(WebSocketStatus::Error("Failed to create WebSocket".to_string()));
            }
        }
    };

    let disconnect = {
        let ws = ws.clone();
        let status = status.clone();
        
        move || {
            if let Some(socket) = (*ws).as_ref() {
                let _ = socket.close();
                status.set(WebSocketStatus::Closing);
            }
        }
    };

    let send_message = {
        let ws = ws.clone();
        let status = status.clone();
        
        move |message: String| {
            if let Some(socket) = (*ws).as_ref() {
                if *status == WebSocketStatus::Open {
                    let _ = socket.send_with_str(&message);
                }
            }
        }
    };

    let reconnect = {
        let ws = ws.clone();
        let status = status.clone();
        let connect = connect.clone();
        
        move || {
            if let Some(socket) = (*ws).as_ref() {
                let _ = socket.close();
                status.set(WebSocketStatus::Closing);
                connect();
            }
        }
    };

    // Connect if not manual
    {
        let connect = connect.clone();
        yew::use_effect_with_deps(
            move |(url, options)| {
                if !options.manual {
                    connect();
                }
                
                || {}
            },
            (url, options),
        );
    }

    (
        (*status).clone(),
        Rc::new((*ws).clone().unwrap_or_else(|| WebSocket::new(&url).unwrap())),
        reconnect,
        send_message,
        disconnect,
    )
}