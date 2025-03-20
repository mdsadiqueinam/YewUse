use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{window, Event, Navigator};
use yew::prelude::*;

/// Information about the network state
#[derive(Debug, Clone, PartialEq)]
pub struct NetworkState {
    /// Whether the browser is online
    pub online: bool,
    /// Network connection type
    pub connection_type: Option<String>,
    /// Network connection effective type (slow-2g, 2g, 3g, 4g)
    pub effective_type: Option<String>,
    /// Network connection downlink speed in Mbps
    pub downlink: Option<f64>,
    /// Network connection RTT in ms
    pub rtt: Option<f64>,
    /// Whether the connection is being saved via data saver
    pub save_data: Option<bool>,
}

impl Default for NetworkState {
    fn default() -> Self {
        Self {
            online: true,
            connection_type: None,
            effective_type: None,
            downlink: None,
            rtt: None,
            save_data: None,
        }
    }
}

/// Hook for monitoring the user's network connection status
///
/// # Returns
///
/// The current network state information
///
/// # Example
///
/// ```rust
/// use yewuse::sensors::use_network_state;
/// use yew::prelude::*;
///
/// #[function_component(NetworkInfo)]
/// fn network_info() -> Html {
///     let network = use_network_state();
///     
///     html! {
///         <div>
///             <h3>{"Network Information"}</h3>
///             <p>{"Online: "}{network.online.to_string()}</p>
///             if let Some(type_name) = &network.connection_type {
///                 <p>{"Connection Type: "}{type_name}</p>
///             }
///             if let Some(effective_type) = &network.effective_type {
///                 <p>{"Effective Type: "}{effective_type}</p>
///             }
///             if let Some(downlink) = network.downlink {
///                 <p>{"Downlink: "}{format!("{:.2} Mbps", downlink)}</p>
///             }
///             if let Some(rtt) = network.rtt {
///                 <p>{"RTT: "}{format!("{} ms", rtt)}</p>
///             }
///             if let Some(save_data) = network.save_data {
///                 <p>{"Data Saver: "}{save_data.to_string()}</p>
///             }
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_network_state() -> NetworkState {
    let network_state = use_state(|| NetworkState::default());
    
    // Initialize the network state and set up listeners
    use_effect_with_deps(
        |_| {
            let window = window().unwrap();
            let network_state_clone = network_state.clone();
            
            // Function to update the network state
            let update_network_state = {
                let network_state_clone = network_state_clone.clone();
                
                move || {
                    let navigator = window.navigator();
                    let is_online = navigator.online().unwrap_or(true);
                    
                    // Try to get detailed connection information
                    let connection_info = get_connection_info(&navigator);
                    
                    network_state_clone.set(NetworkState {
                        online: is_online,
                        connection_type: connection_info.0,
                        effective_type: connection_info.1,
                        downlink: connection_info.2,
                        rtt: connection_info.3,
                        save_data: connection_info.4,
                    });
                }
            };
            
            // Initial update
            update_network_state();
            
            // Set up online/offline event listeners
            let online_callback = {
                let network_state_clone = network_state_clone.clone();
                Closure::wrap(Box::new(move |_: Event| {
                    let mut state = (*network_state_clone).clone();
                    state.online = true;
                    network_state_clone.set(state);
                }) as Box<dyn FnMut(_)>)
            };
            
            let offline_callback = {
                let network_state_clone = network_state_clone.clone();
                Closure::wrap(Box::new(move |_: Event| {
                    let mut state = (*network_state_clone).clone();
                    state.online = false;
                    network_state_clone.set(state);
                }) as Box<dyn FnMut(_)>)
            };
            
            window
                .add_event_listener_with_callback("online", online_callback.as_ref().unchecked_ref())
                .unwrap();
            
            window
                .add_event_listener_with_callback("offline", offline_callback.as_ref().unchecked_ref())
                .unwrap();
            
            // Set up connection change listener if supported
            let navigator = window.navigator();
            let mut connection_change_callback = None;
            
            // Check if the Connection API is available
            if js_sys::Reflect::has(&navigator, &"connection".into()).unwrap_or(false) {
                let connection = js_sys::Reflect::get(&navigator, &"connection".into()).ok();
                
                if let Some(connection) = connection {
                    if js_sys::Reflect::has(&connection, &"addEventListener".into()).unwrap_or(false) {
                        let callback = {
                            let network_state_clone = network_state_clone.clone();
                            Closure::wrap(Box::new(move |_: Event| {
                                update_network_state();
                            }) as Box<dyn FnMut(_)>)
                        };
                        
                        let _ = js_sys::Reflect::apply(
                            &js_sys::Reflect::get(&connection, &"addEventListener".into()).unwrap(),
                            &connection,
                            &js_sys::Array::of2(
                                &"change".into(),
                                callback.as_ref().unchecked_ref(),
                            ),
                        );
                        
                        connection_change_callback = Some((connection, callback));
                    }
                }
            }
            
            // Return a cleanup function
            move || {
                window
                    .remove_event_listener_with_callback("online", online_callback.as_ref().unchecked_ref())
                    .unwrap();
                
                window
                    .remove_event_listener_with_callback("offline", offline_callback.as_ref().unchecked_ref())
                    .unwrap();
                
                // Remove connection change listener if it was set up
                if let Some((connection, callback)) = connection_change_callback {
                    let _ = js_sys::Reflect::apply(
                        &js_sys::Reflect::get(&connection, &"removeEventListener".into()).unwrap(),
                        &connection,
                        &js_sys::Array::of2(
                            &"change".into(),
                            callback.as_ref().unchecked_ref(),
                        ),
                    );
                }
            }
        },
        (),
    );
    
    (*network_state).clone()
}

/// Helper function to extract connection information
fn get_connection_info(navigator: &Navigator) -> (
    Option<String>,     // connection_type
    Option<String>,     // effective_type
    Option<f64>,        // downlink
    Option<f64>,        // rtt
    Option<bool>,       // save_data
) {
    // Check if the Connection API is available
    if js_sys::Reflect::has(navigator, &"connection".into()).unwrap_or(false) {
        if let Ok(connection) = js_sys::Reflect::get(navigator, &"connection") {
            let connection_type = js_sys::Reflect::get(&connection, &"type".into())
                .ok()
                .and_then(|v| v.as_string());
            
            let effective_type = js_sys::Reflect::get(&connection, &"effectiveType".into())
                .ok()
                .and_then(|v| v.as_string());
            
            let downlink = js_sys::Reflect::get(&connection, &"downlink".into())
                .ok()
                .and_then(|v| v.as_f64());
            
            let rtt = js_sys::Reflect::get(&connection, &"rtt".into())
                .ok()
                .and_then(|v| v.as_f64());
            
            let save_data = js_sys::Reflect::get(&connection, &"saveData".into())
                .ok()
                .and_then(|v| v.as_bool());
            
            return (connection_type, effective_type, downlink, rtt, save_data);
        }
    }
    
    (None, None, None, None, None)
}