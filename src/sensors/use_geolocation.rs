use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{window, GeolocationPosition, GeolocationPositionError};
use yew::prelude::*;

/// Geolocation data that includes coordinates and accuracy
#[derive(Clone, Debug, PartialEq)]
pub struct GeolocationData {
    /// Latitude in decimal degrees
    pub latitude: f64,
    /// Longitude in decimal degrees
    pub longitude: f64,
    /// Accuracy of the coordinates in meters
    pub accuracy: f64,
    /// Altitude in meters, if available
    pub altitude: Option<f64>,
    /// Accuracy of the altitude in meters, if available
    pub altitude_accuracy: Option<f64>,
    /// Direction of travel in degrees, if available
    pub heading: Option<f64>,
    /// Velocity in meters per second, if available
    pub speed: Option<f64>,
    /// Timestamp when the position was retrieved
    pub timestamp: f64,
}

/// Error states that can occur during geolocation
#[derive(Clone, Debug, PartialEq)]
pub enum GeolocationError {
    /// User denied permission to access location
    PermissionDenied(String),
    /// Location information unavailable
    PositionUnavailable(String),
    /// Network timeout
    Timeout(String),
    /// Browser doesn't support geolocation
    Unsupported,
    /// Other error
    Other(String),
}

/// Options for configuring the geolocation API
#[derive(Clone, PartialEq)]
pub struct GeolocationOptions {
    /// Is high accuracy enabled
    pub enable_high_accuracy: bool,
    /// Time in milliseconds that is allowed to return a position
    pub timeout: Option<i32>,
    /// Maximum age in milliseconds of a cached position
    pub maximum_age: Option<i32>,
    /// Whether to watch position continuously
    pub watch: bool,
}

impl Default for GeolocationOptions {
    fn default() -> Self {
        Self {
            enable_high_accuracy: false,
            timeout: None,
            maximum_age: None,
            watch: false,
        }
    }
}

/// Hook to access the user's geolocation
///
/// # Arguments
///
/// * `options` - Optional configuration for the geolocation API
///
/// # Returns
///
/// A tuple containing:
/// - Whether location is currently being retrieved
/// - The location data, if available
/// - Any error that occurred during retrieval
/// - Function to refresh the location data
///
/// # Example
///
/// ```rust
/// use yewuse::sensors::{use_geolocation, GeolocationOptions};
/// use yew::prelude::*;
///
/// #[function_component(LocationDisplay)]
/// fn location_display() -> Html {
///     let options = GeolocationOptions {
///         enable_high_accuracy: true,
///         ..Default::default()
///     };
///     
///     let (is_loading, location, error, refresh) = use_geolocation(Some(options));
///     
///     html! {
///         <div>
///             <h2>{"Your Location"}</h2>
///             
///             {if is_loading {
///                 html! { <p>{"Getting your location..."}</p> }
///             } else if let Some(error) = &error {
///                 html! { <p class="error">{"Error: "}{format!("{:?}", error)}</p> }
///             } else if let Some(loc) = &location {
///                 html! {
///                     <div>
///                         <p>{"Latitude: "}{loc.latitude}</p>
///                         <p>{"Longitude: "}{loc.longitude}</p>
///                         <p>{"Accuracy: "}{loc.accuracy}{" meters"}</p>
///                         if let Some(alt) = loc.altitude {
///                             <p>{"Altitude: "}{alt}{" meters"}</p>
///                         }
///                     </div>
///                 }
///             } else {
///                 html! { <p>{"No location data available"}</p> }
///             }}
///             
///             <button onclick={refresh}>{"Refresh Location"}</button>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_geolocation(
    options: Option<GeolocationOptions>,
) -> (
    bool,                        // is_loading
    Option<GeolocationData>,     // location
    Option<GeolocationError>,    // error
    Callback<()>,                // refresh function
) {
    let options = options.unwrap_or_default();
    let is_loading = use_state(|| false);
    let location = use_state(|| None::<GeolocationData>);
    let error = use_state(|| None::<GeolocationError>);
    let watch_id = use_mut_ref(|| None::<i32>);
    
    // Check if geolocation is supported
    let is_supported = window()
        .and_then(|w| w.navigator().geolocation().ok())
        .is_some();
    
    // Handle position success
    let handle_success = {
        let is_loading = is_loading.clone();
        let location = location.clone();
        
        Closure::wrap(Box::new(move |pos: GeolocationPosition| {
            let coords = pos.coords();
            
            let geo_data = GeolocationData {
                latitude: coords.latitude(),
                longitude: coords.longitude(),
                accuracy: coords.accuracy(),
                altitude: match coords.altitude_f64() {
                    Ok(alt) if !alt.is_nan() => Some(alt),
                    _ => None,
                },
                altitude_accuracy: match coords.altitude_accuracy_f64() {
                    Ok(acc) if !acc.is_nan() => Some(acc),
                    _ => None,
                },
                heading: match coords.heading_f64() {
                    Ok(heading) if !heading.is_nan() => Some(heading),
                    _ => None,
                },
                speed: match coords.speed_f64() {
                    Ok(speed) if !speed.is_nan() => Some(speed),
                    _ => None,
                },
                timestamp: pos.timestamp(),
            };
            
            location.set(Some(geo_data));
            is_loading.set(false);
        }) as Box<dyn FnMut(_)>)
    };
    
    // Handle position error
    let handle_error = {
        let is_loading = is_loading.clone();
        let error = error.clone();
        
        Closure::wrap(Box::new(move |err: GeolocationPositionError| {
            let geo_error = match err.code() {
                1 => GeolocationError::PermissionDenied(err.message()),
                2 => GeolocationError::PositionUnavailable(err.message()),
                3 => GeolocationError::Timeout(err.message()),
                _ => GeolocationError::Other(err.message()),
            };
            
            error.set(Some(geo_error));
            is_loading.set(false);
        }) as Box<dyn FnMut(_)>)
    };
    
    // Function to get current position
    let get_current_position = {
        let is_loading = is_loading.clone();
        let error = error.clone();
        let handle_success = handle_success.clone();
        let handle_error = handle_error.clone();
        let options = options.clone();
        
        move || {
            if !is_supported {
                error.set(Some(GeolocationError::Unsupported));
                return;
            }
            
            let window = window().unwrap();
            let geolocation = window.navigator().geolocation().unwrap();
            
            // Create position options
            let mut position_options = web_sys::PositionOptions::new();
            position_options.enable_high_accuracy(options.enable_high_accuracy);
            
            if let Some(timeout) = options.timeout {
                position_options.timeout(timeout);
            }
            
            if let Some(max_age) = options.maximum_age {
                position_options.maximum_age(max_age);
            }
            
            // Start loading
            is_loading.set(true);
            error.set(None);
            
            // Get position
            if options.watch {
                if watch_id.borrow().is_some() {
                    // Already watching, do nothing
                    return;
                }
                
                match geolocation.watch_position_with_error_callback_and_options(
                    handle_success.as_ref().unchecked_ref(),
                    handle_error.as_ref().unchecked_ref(), 
                    &position_options,
                ) {
                    Ok(id) => {
                        *watch_id.borrow_mut() = Some(id);
                    }
                    Err(_) => {
                        error.set(Some(GeolocationError::Other("Failed to watch position".to_string())));
                        is_loading.set(false);
                    }
                }
            } else {
                let _ = geolocation.get_current_position_with_error_callback_and_options(
                    handle_success.as_ref().unchecked_ref(),
                    handle_error.as_ref().unchecked_ref(),
                    &position_options,
                );
            }
        }
    };
    
    // Refresh function
    let refresh = {
        let get_current_position = get_current_position.clone();
        
        Callback::from(move |_| {
            get_current_position();
        })
    };
    
    // Initial position request and cleanup
    {
        let get_current_position = get_current_position.clone();
        let watch_id = watch_id.clone();
        
        use_effect_with_deps(
            move |_| {
                // Request initial position
                get_current_position();
                
                // Cleanup function
                move || {
                    // Clear watch if active
                    if let Some(id) = *watch_id.borrow() {
                        if let Some(window) = window() {
                            if let Ok(geolocation) = window.navigator().geolocation() {
                                geolocation.clear_watch(id);
                            }
                        }
                    }
                }
            },
            (),
        );
    }
    
    (
        *is_loading,
        (*location).clone(),
        (*error).clone(),
        refresh,
    )
}