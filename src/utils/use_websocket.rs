use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{MessageEvent, WebSocket};
use yew::prelude::*;
use std::rc::Rc;

/// WebSocket connection status
#[derive(Debug, Clone, PartialEq)]
pub enum WebSocketStatus {
    /// Connection is being established
    Connecting,
    /// Connection is open and ready to communicate
    Open,
    /// Connection is closing
    Closing,
    /// Connection is closed or couldn't be opened
    Closed,
    /// Error occurred
    Error(String),
}

impl Default for WebSocketStatus {
    fn default() -> Self {
        Self::Closed
    }
}

/// Options for configuring the WebSocket connection
#[derive(Clone, PartialEq)]
pub struct UseWebSocketOptions {
    /// Whether to auto-reconnect on connection close/error
    pub auto_reconnect: bool,
    /// Maximum number of reconnection attempts
    pub max_reconnect_attempts: Option<usize>,
    /// Delay in milliseconds between reconnection attempts
    pub reconnect_delay: u32,
    /// Whether to automatically connect on hook initialization
    pub immediate: bool,
    /// WebSocket protocols to use
    pub protocols: Option<Vec<String>>,
}

impl Default for UseWebSocketOptions {
    fn default() -> Self {
        Self {
            auto_reconnect: true,
            max_reconnect_attempts: Some(3),
            reconnect_delay: 1000,
            immediate: true,
            protocols: None,
        }
    }
}

/// Hook for managing WebSocket connections
///
/// # Arguments
///
/// * `url` - WebSocket server URL (starting with ws:// or wss://)
/// * `options` - Optional configuration for the connection
///
/// # Returns
///
/// A tuple containing:
/// - Current connection status
/// - Last received message data
/// - Function to send a message
/// - Function to connect manually
/// - Function to disconnect manually
///
/// # Example
///
/// ```rust
/// use yewuse::utils::{use_websocket, WebSocketStatus, UseWebSocketOptions};
/// use yew::prelude::*;
///
/// #[function_component(WebSocketChat)]
/// fn websocket_chat() -> Html {
///     let input_ref = use_node_ref();
///     let options = UseWebSocketOptions {
///         auto_reconnect: true,
///         immediate: true,
///         ..Default::default()
///     };
///     
///     let (status, message, send, connect, disconnect) = 
///         use_websocket("wss://echo.websocket.org".to_string(), Some(options));
///     
///     let on_send = {
///         let input_ref = input_ref.clone();
///         let send = send.clone();
///         
///         Callback::from(move |_| {
///             if let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>() {
///                 let message = input.value();
///                 if !message.is_empty() {
///                     send(message.clone());
///                     input.set_value("");
///                 }
///             }
///         })
///     };
///     
///     let status_text = match &status {
///         WebSocketStatus::Connecting => "Connecting...",
///         WebSocketStatus::Open => "Connected",
///         WebSocketStatus::Closing => "Closing...",
///         WebSocketStatus::Closed => "Disconnected",
///         WebSocketStatus::Error(err) => err,
///     };
///     
///     html! {
///         <div>
///             <div>{"Status: "}{status_text}</div>
///             
///             <div>
///                 <button onclick={connect} disabled={status == WebSocketStatus::Open}>
///                     {"Connect"}
///                 </button>
///                 <button onclick={disconnect} disabled={status != WebSocketStatus::Open}>
///                     {"Disconnect"}
///                 </button>
///             </div>
///             
///             <div>
///                 <input 
///                     ref={input_ref}
///                     type="text" 
///                     placeholder="Type message..." 
///                     disabled={status != WebSocketStatus::Open}
///                 />
///                 <button 
///                     onclick={on_send}
///                     disabled={status != WebSocketStatus::Open}
///                 >
///                     {"Send"}
///                 </button>
///             </div>
///             
///             <div>
///                 <h3>{"Received Message:"}</h3>
///                 <p>{message.clone()}</p>
///             </div>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_websocket(
    url: String,
    options: Option<UseWebSocketOptions>,
) -> (
    WebSocketStatus,                 // status
    UseStateHandle<String>,          // message
    Callback<String>,                // send
    Callback<()>,                    // connect
    Callback<()>,                    // disconnect
) {
    let options = options.unwrap_or_default();
    let status = use_state(WebSocketStatus::default);
    let message = use_state(String::default);
    let websocket = use_mut_ref(|| None::<WebSocket>);
    let reconnect_attempts = use_mut_ref(|| 0_usize);
    let reconnect_timer = use_mut_ref(|| None::<i32>);
    
    // Function to create and set up a WebSocket connection
    let setup_websocket = {
        let url = url.clone();
        let options = options.clone();
        let status = status.clone();
        let message = message.clone();
        let websocket = websocket.clone();
        let reconnect_attempts = reconnect_attempts.clone();
        
        Rc::new(move || {
            // Reset WebSocket if one already exists
            if let Some(ws) = websocket.borrow().as_ref() {
                if ws.ready_state() != WebSocket::CLOSED {
                    let _ = ws.close();
                }
            }
            
            // Update status to connecting
            status.set(WebSocketStatus::Connecting);
            
            // Create a new WebSocket
            let ws = match &options.protocols {
                Some(protocols) => {
                    let protocols_arr = protocols.join(",");
                    match WebSocket::new_with_str(&url, &protocols_arr) {
                        Ok(ws) => ws,
                        Err(err) => {
                            status.set(WebSocketStatus::Error(format!("Failed to create WebSocket: {:?}", err)));
                            return;
                        }
                    }
                },
                None => {
                    match WebSocket::new(&url) {
                        Ok(ws) => ws,
                        Err(err) => {
                            status.set(WebSocketStatus::Error(format!("Failed to create WebSocket: {:?}", err)));
                            return;
                        }
                    }
                }
            };
            
            // Set up event handlers
            
            // Open handler
            let open_status = status.clone();
            let open_callback = Closure::wrap(Box::new(move |_| {
                open_status.set(WebSocketStatus::Open);
                *reconnect_attempts.borrow_mut() = 0; // Reset reconnect attempts on successful connection
            }) as Box<dyn FnMut(JsValue)>);
            ws.set_onopen(Some(open_callback.as_ref().unchecked_ref()));
            open_callback.forget();
            
            // Message handler
            let message_clone = message.clone();
            let message_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
                if let Ok(data) = e.data().as_string() {
                    message_clone.set(data);
                }
            }) as Box<dyn FnMut(MessageEvent)>);
            ws.set_onmessage(Some(message_callback.as_ref().unchecked_ref()));
            message_callback.forget();
            
            // Error handler
            let error_status = status.clone();
            let error_callback = Closure::wrap(Box::new(move |_| {
                error_status.set(WebSocketStatus::Error("WebSocket error occurred".to_string()));
            }) as Box<dyn FnMut(JsValue)>);
            ws.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
            error_callback.forget();
            
            // Close handler
            let close_status = status.clone();
            let close_ws = websocket.clone();
            let close_reconnect_attempts = reconnect_attempts.clone();
            let close_options = options.clone();
            let close_url = url.clone();
            let close_reconnect_timer = reconnect_timer.clone();
            
            let close_callback = Closure::wrap(Box::new(move |_| {
                close_status.set(WebSocketStatus::Closed);
                
                // Handle reconnection if configured
                if close_options.auto_reconnect {
                    let current_attempts = *close_reconnect_attempts.borrow();
                    let max_attempts = close_options.max_reconnect_attempts.unwrap_or(usize::MAX);
                    
                    if current_attempts < max_attempts {
                        // Increment attempts counter
                        *close_reconnect_attempts.borrow_mut() = current_attempts + 1;
                        
                        // Schedule reconnection
                        let window = web_sys::window().unwrap();
                        let close_url = close_url.clone();
                        let close_ws = close_ws.clone();
                        let close_status = close_status.clone();
                        
                        let reconnect_callback = Closure::once(Box::new(move || {
                            // Attempt to reconnect
                            let reconnect_ws = match WebSocket::new(&close_url) {
                                Ok(ws) => ws,
                                Err(_) => {
                                    close_status.set(WebSocketStatus::Error("Reconnection failed".to_string()));
                                    return;
                                }
                            };
                            *close_ws.borrow_mut() = Some(reconnect_ws);
                        }) as Box<dyn FnOnce()>);
                        
                        let handle = window
                            .set_timeout_with_callback_and_timeout_and_arguments_0(
                                reconnect_callback.as_ref().unchecked_ref(),
                                close_options.reconnect_delay as i32,
                            )
                            .unwrap();
                        
                        *close_reconnect_timer.borrow_mut() = Some(handle);
                        reconnect_callback.forget();
                    }
                }
            }) as Box<dyn FnMut(JsValue)>);
            
            ws.set_onclose(Some(close_callback.as_ref().unchecked_ref()));
            close_callback.forget();
            
            // Store the WebSocket
            *websocket.borrow_mut() = Some(ws);
        })
    };
    
    // Connect function
    let connect = {
        let setup_websocket = setup_websocket.clone();
        
        Callback::from(move |_| {
            setup_websocket();
        })
    };
    
    // Disconnect function
    let disconnect = {
        let status = status.clone();
        let websocket = websocket.clone();
        let reconnect_timer = reconnect_timer.clone();
        
        Callback::from(move |_| {
            // Clear any pending reconnect timer
            if let Some(handle) = *reconnect_timer.borrow() {
                web_sys::window().unwrap().clear_timeout_with_handle(handle);
                *reconnect_timer.borrow_mut() = None;
            }
            
            if let Some(ws) = websocket.borrow().as_ref() {
                if ws.ready_state() == WebSocket::OPEN || ws.ready_state() == WebSocket::CONNECTING {
                    status.set(WebSocketStatus::Closing);
                    let _ = ws.close();
                }
            }
        })
    };
    
    // Send function
    let send = {
        let websocket = websocket.clone();
        let status = status.clone();
        
        Callback::from(move |data: String| {
            if let Some(ws) = websocket.borrow().as_ref() {
                if ws.ready_state() == WebSocket::OPEN {
                    if let Err(_) = ws.send_with_str(&data) {
                        status.set(WebSocketStatus::Error("Failed to send message".to_string()));
                    }
                }
            }
        })
    };
    
    // Set up initial connection if requested
    {
        let connect = connect.clone();
        
        use_effect_with_deps(
            move |(url, options)| {
                if options.immediate {
                    connect.emit(());
                }
                
                move || {
                    // Clean up on unmount
                    if let Some(ws) = websocket.borrow().as_ref() {
                        if ws.ready_state() != WebSocket::CLOSED {
                            let _ = ws.close();
                        }
                    }
                    
                    // Clear any reconnect timer
                    if let Some(handle) = *reconnect_timer.borrow() {
                        web_sys::window().unwrap().clear_timeout_with_handle(handle);
                    }
                }
            },
            (url, options),
        );
    }
    
    ((*status).clone(), message, send, connect, disconnect)
}