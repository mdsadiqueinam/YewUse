use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{window, DeviceOrientationEvent};
use yew::prelude::*;

/// Structure to hold device orientation information
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceOrientation {
    /// Alpha value: rotation around the z-axis in degrees (0-360)
    pub alpha: Option<f64>,
    /// Beta value: front-to-back tilt in degrees (-180-180)
    pub beta: Option<f64>,
    /// Gamma value: left-to-right tilt in degrees (-90-90)
    pub gamma: Option<f64>,
    /// Whether the device supports orientation events
    pub is_supported: bool,
}

/// Hook for tracking device orientation
///
/// # Returns
///
/// The current device orientation information
///
/// # Example
///
/// ```rust
/// use yewuse::sensors::use_device_orientation;
///
/// #[function_component(OrientationTracker)]
/// fn orientation_tracker() -> Html {
///     let orientation = use_device_orientation();
///     
///     html! {
///         <div>
///             if orientation.is_supported {
///                 <div>
///                     <p>{"Alpha: "}{orientation.alpha.map_or("N/A".to_string(), |v| format!("{:.1}°", v))}</p>
///                     <p>{"Beta: "}{orientation.beta.map_or("N/A".to_string(), |v| format!("{:.1}°", v))}</p>
///                     <p>{"Gamma: "}{orientation.gamma.map_or("N/A".to_string(), |v| format!("{:.1}°", v))}</p>
///                 </div>
///             } else {
///                 <p>{"Device orientation not supported"}</p>
///             }
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_device_orientation() -> DeviceOrientation {
    let orientation = use_state(|| DeviceOrientation {
        alpha: None,
        beta: None,
        gamma: None,
        is_supported: false,
    });

    use_effect_with_deps(
        |_| {
            let orientation_clone = orientation.clone();
            let window = window().unwrap();
            
            // Check if device orientation is supported
            let is_supported = js_sys::Reflect::get(&window, &"DeviceOrientationEvent".into()).is_ok();
            
            if !is_supported {
                orientation_clone.set(DeviceOrientation {
                    alpha: None,
                    beta: None,
                    gamma: None,
                    is_supported: false,
                });
                return || {};
            }
            
            let callback = Closure::wrap(Box::new(move |e: DeviceOrientationEvent| {
                orientation_clone.set(DeviceOrientation {
                    alpha: e.alpha(),
                    beta: e.beta(),
                    gamma: e.gamma(),
                    is_supported: true,
                });
            }) as Box<dyn FnMut(_)>);

            window
                .add_event_listener_with_callback("deviceorientation", callback.as_ref().unchecked_ref())
                .unwrap();

            // Return cleanup function
            move || {
                if is_supported {
                    window
                        .remove_event_listener_with_callback("deviceorientation", callback.as_ref().unchecked_ref())
                        .unwrap();
                }
            }
        },
        (),
    );

    (*orientation).clone()
}