use wasm_bindgen::JsCast;
use web_sys::{window, NetworkInformation};
use yew::hook;

#[derive(Clone, Debug, PartialEq)]
pub struct NetworkState {
    pub online: bool,
    pub downlink: Option<f64>,
    pub downlink_max: Option<f64>,
    pub effective_type: Option<String>,
    pub rtt: Option<f64>,
    pub save_data: bool,
    pub type_: Option<String>,
}

impl Default for NetworkState {
    fn default() -> Self {
        Self {
            online: true,
            downlink: None,
            downlink_max: None,
            effective_type: None,
            rtt: None,
            save_data: false,
            type_: None,
        }
    }
}

#[hook]
pub fn use_network_state() -> NetworkState {
    let state = yew::use_state(NetworkState::default);

    {
        let state = state.clone();
        yew::use_effect_with_deps(
            move |_| {
                let window = window().unwrap();
                let navigator = window.navigator();
                let connection = navigator.as_ref()
                    .dyn_ref::<NetworkInformation>();
                
                // Initial state
                state.set(NetworkState {
                    online: navigator.on_line(),
                    downlink: connection
                        .and_then(|conn| conn.downlink().ok()),
                    downlink_max: connection
                        .and_then(|conn| conn.downlink_max().ok()),
                    effective_type: connection
                        .and_then(|conn| conn.effective_type().ok()),
                    rtt: connection
                        .and_then(|conn| conn.rtt().ok()),
                    save_data: connection
                        .and_then(|conn| conn.save_data().ok())
                        .unwrap_or(false),
                    type_: connection
                        .and_then(|conn| conn.type_().ok()),
                });

                // Online status handler
                let online_callback = {
                    let state = state.clone();
                    let cb = move || {
                        let mut current = (*state).clone();
                        current.online = true;
                        state.set(current);
                    };
                    wasm_bindgen::closure::Closure::wrap(
                        Box::new(cb) as Box<dyn FnMut()>
                    )
                };

                // Offline status handler
                let offline_callback = {
                    let state = state.clone();
                    let cb = move || {
                        let mut current = (*state).clone();
                        current.online = false;
                        state.set(current);
                    };
                    wasm_bindgen::closure::Closure::wrap(
                        Box::new(cb) as Box<dyn FnMut()>
                    )
                };

                // Connection change handler
                let connection_change = {
                    let state = state.clone();
                    let cb = move || {
                        let navigator = window.navigator();
                        let mut current = (*state).clone();
                        
                        if let Some(conn) = navigator.as_ref().dyn_ref::<NetworkInformation>() {
                            current.downlink = conn.downlink().ok();
                            current.downlink_max = conn.downlink_max().ok();
                            current.effective_type = conn.effective_type().ok();
                            current.rtt = conn.rtt().ok();
                            current.save_data = conn.save_data().ok().unwrap_or(false);
                            current.type_ = conn.type_().ok();
                        }
                        
                        state.set(current);
                    };
                    wasm_bindgen::closure::Closure::wrap(
                        Box::new(cb) as Box<dyn FnMut()>
                    )
                };

                // Add event listeners
                window
                    .add_event_listener_with_callback("online", online_callback.as_ref().unchecked_ref())
                    .unwrap();
                window
                    .add_event_listener_with_callback("offline", offline_callback.as_ref().unchecked_ref())
                    .unwrap();

                if let Some(conn) = navigator.as_ref().dyn_ref::<NetworkInformation>() {
                    conn.add_event_listener_with_callback(
                        "change",
                        connection_change.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                }

                move || {
                    window
                        .remove_event_listener_with_callback("online", online_callback.as_ref().unchecked_ref())
                        .unwrap();
                    window
                        .remove_event_listener_with_callback("offline", offline_callback.as_ref().unchecked_ref())
                        .unwrap();

                    if let Some(conn) = navigator.as_ref().dyn_ref::<NetworkInformation>() {
                        conn.remove_event_listener_with_callback(
                            "change",
                            connection_change.as_ref().unchecked_ref(),
                        )
                        .unwrap();
                    }
                }
            },
            (),
        );
    }

    (*state).clone()
}