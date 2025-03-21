use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{window, Geolocation};
use js_sys::Object;
use wasm_bindgen::JsValue;
use yew::hook;

#[derive(Clone)]
pub struct GeolocationState {
    pub loading: bool,
    pub error: Option<GeolocationError>,
    pub position: Option<Position>,
}

#[derive(Clone)]
pub struct Position {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: f64,
    pub altitude: Option<f64>,
    pub altitude_accuracy: Option<f64>,
    pub heading: Option<f64>,
    pub speed: Option<f64>,
    pub timestamp: f64,
}

#[derive(Clone)]
pub struct GeolocationError {
    pub code: i16,
    pub message: String,
}

#[derive(Clone)]
pub struct UseGeolocationOptions {
    pub enable_high_accuracy: bool,
    pub timeout: Option<i32>,
    pub maximum_age: Option<i32>,
    pub watch: bool,
    pub continue_updates: bool,  // New field to control continuous updates
}

impl Default for UseGeolocationOptions {
    fn default() -> Self {
        Self {
            enable_high_accuracy: false,
            timeout: None,
            maximum_age: None,
            watch: false,
            continue_updates: true,  // Default to continuing updates
        }
    }
}

impl Default for GeolocationState {
    fn default() -> Self {
        Self {
            loading: true,
            error: None,
            position: None,
        }
    }
}

#[hook]
pub fn use_geolocation(options: Option<UseGeolocationOptions>) -> GeolocationState {
    let options = options.unwrap_or_default();
    let state = yew::use_state(GeolocationState::default);
    let watch_id = yew::use_mut_ref(|| None::<i32>);
    let first_position_received = yew::use_mut_ref(|| false);

    {
        let state = state.clone();
        let watch_id = watch_id.clone();
        let first_position_received = first_position_received.clone();
        
        yew::use_effect_with_deps(
            move |(options,)| {
                let window = window().expect("no window");
                let navigator = window.navigator();
                let geolocation = navigator.geolocation().expect("no geolocation");

                // Success callback
                let success_callback = {
                    let state = state.clone();
                    let watch_id = watch_id.clone();
                    let first_position_received = first_position_received.clone();
                    
                    Closure::wrap(Box::new(move |pos: JsValue| {
                        // Extract position data from JsValue
                        let coords = js_sys::Reflect::get(&pos, &JsValue::from_str("coords"))
                            .expect("no coords in position");
                        
                        // Extract timestamp directly
                        let timestamp = js_sys::Reflect::get(&pos, &JsValue::from_str("timestamp"))
                            .expect("no timestamp in position")
                            .as_f64()
                            .unwrap_or(0.0);
                        
                        // Extract coordinates
                        let latitude = js_sys::Reflect::get(&coords, &JsValue::from_str("latitude"))
                            .expect("no latitude")
                            .as_f64()
                            .unwrap_or(0.0);
                        let longitude = js_sys::Reflect::get(&coords, &JsValue::from_str("longitude"))
                            .expect("no longitude")
                            .as_f64()
                            .unwrap_or(0.0);
                        let accuracy = js_sys::Reflect::get(&coords, &JsValue::from_str("accuracy"))
                            .expect("no accuracy")
                            .as_f64()
                            .unwrap_or(0.0);
                            
                        // Optional coordinates
                        let altitude = js_sys::Reflect::get(&coords, &JsValue::from_str("altitude"))
                            .ok()
                            .and_then(|v| v.as_f64());
                        let altitude_accuracy = js_sys::Reflect::get(&coords, &JsValue::from_str("altitudeAccuracy"))
                            .ok()
                            .and_then(|v| v.as_f64());
                        let heading = js_sys::Reflect::get(&coords, &JsValue::from_str("heading"))
                            .ok()
                            .and_then(|v| v.as_f64());
                        let speed = js_sys::Reflect::get(&coords, &JsValue::from_str("speed"))
                            .ok()
                            .and_then(|v| v.as_f64());
                            
                        state.set(GeolocationState {
                            loading: false,
                            error: None,
                            position: Some(Position {
                                latitude,
                                longitude,
                                accuracy,
                                altitude,
                                altitude_accuracy,
                                heading,
                                speed,
                                timestamp,
                            }),
                        });

                        // If this is the first position and we don't want to continue updates
                        if !*first_position_received.borrow() {
                            *first_position_received.borrow_mut() = true;
                            
                            if !options.continue_updates && options.watch {
                                // Clear the watch if we don't want to continue
                                if let Some(id) = *watch_id.borrow() {
                                    geolocation.clear_watch(id);
                                    *watch_id.borrow_mut() = None;
                                }
                            }
                        }
                    }) as Box<dyn FnMut(JsValue)>)
                };

                // Error callback
                let error_callback = {
                    let state = state.clone();
                    Closure::wrap(Box::new(move |err: JsValue| {
                        // Extract error data
                        let code = js_sys::Reflect::get(&err, &JsValue::from_str("code"))
                            .expect("no error code")
                            .as_f64()
                            .unwrap_or(0.0) as i16;
                            
                        let message = js_sys::Reflect::get(&err, &JsValue::from_str("message"))
                            .expect("no error message")
                            .as_string()
                            .unwrap_or_else(|| String::from("Unknown geolocation error"));
                            
                        state.set(GeolocationState {
                            loading: false,
                            error: Some(GeolocationError {
                                code,
                                message,
                            }),
                            position: None,
                        });
                    }) as Box<dyn FnMut(JsValue)>)
                };

                // Set up options as a JavaScript object
                let geo_options = Object::new();
                js_sys::Reflect::set(
                    &geo_options,
                    &JsValue::from_str("enableHighAccuracy"),
                    &JsValue::from_bool(options.enable_high_accuracy),
                ).expect("Could not set enableHighAccuracy");
                
                if let Some(timeout) = options.timeout {
                    js_sys::Reflect::set(
                        &geo_options,
                        &JsValue::from_str("timeout"),
                        &JsValue::from_f64(timeout as f64),
                    ).expect("Could not set timeout");
                }
                
                if let Some(max_age) = options.maximum_age {
                    js_sys::Reflect::set(
                        &geo_options,
                        &JsValue::from_str("maximumAge"),
                        &JsValue::from_f64(max_age as f64),
                    ).expect("Could not set maximumAge");
                }

                // Watch position or get current position
                if options.watch {
                    let id = geolocation
                        .watch_position_with_error_callback_and_options(
                            success_callback.as_ref().unchecked_ref(),
                            Some(error_callback.as_ref().unchecked_ref()),
                            geo_options.unchecked_ref(),
                        )
                        .ok();
                    *watch_id.borrow_mut() = id;
                } else {
                    geolocation
                        .get_current_position_with_error_callback_and_options(
                            success_callback.as_ref().unchecked_ref(),
                            Some(error_callback.as_ref().unchecked_ref()),
                            geo_options.unchecked_ref(),
                        )
                        .unwrap();
                }

                // Keep callbacks alive
                success_callback.forget();
                error_callback.forget();

                // Cleanup
                move || {
                    if let Some(id) = *watch_id.borrow() {
                        geolocation.clear_watch(id);
                    }
                }
            },
            (options,),
        );
    }

    (*state).clone()
}