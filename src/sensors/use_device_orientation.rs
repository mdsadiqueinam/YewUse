use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{window, DeviceOrientationEvent};
use yew::hook;

#[derive(Clone, Debug)]
pub struct DeviceOrientation {
    pub absolute: bool,
    pub alpha: Option<f64>,
    pub beta: Option<f64>,
    pub gamma: Option<f64>,
}

impl Default for DeviceOrientation {
    fn default() -> Self {
        Self {
            absolute: false,
            alpha: None,
            beta: None,
            gamma: None,
        }
    }
}

#[hook]
pub fn use_device_orientation() -> DeviceOrientation {
    let orientation = yew::use_state(DeviceOrientation::default);
    {
        let orientation = orientation.clone();
        yew::use_effect_with_deps(
            move |_| {
                let callback = {
                    let orientation = orientation.clone();
                    Closure::wrap(Box::new(move |event: DeviceOrientationEvent| {
                        orientation.set(DeviceOrientation {
                            absolute: event.absolute(),
                            alpha: event.alpha(),
                            beta: event.beta(),
                            gamma: event.gamma(),
                        });
                    }) as Box<dyn FnMut(DeviceOrientationEvent)>)
                };

                // Store the callback reference for cleanup
                let callback_ref = callback.as_ref().unchecked_ref();
                
                if let Some(window) = window() {
                    window
                        .add_event_listener_with_callback(
                            "deviceorientation",
                            callback_ref,
                        )
                        .unwrap();
                    callback.forget();
                }

                // Return a consistent cleanup function
                Box::new(move || {
                    if let Some(window) = window() {
                        let _ = window.remove_event_listener_with_callback(
                            "deviceorientation",
                            callback_ref,
                        );
                    }
                }) as Box<dyn FnOnce()>
            },
            (),
        );
    }
    (*orientation).clone()
}