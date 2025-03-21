use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::window;
use yew::prelude::*;

/// Options for the use_idle hook
#[derive(Clone, PartialEq)]
pub struct UseIdleOptions {
    /// Events to listen for to detect user activity
    pub events: Vec<String>,
    /// Initial state - whether the user is idle initially
    pub initial_state: bool,
    /// Timeout in milliseconds before considering the user idle
    pub timeout: u32,
}

impl Default for UseIdleOptions {
    fn default() -> Self {
        Self {
            events: vec![
                "mousemove".to_string(),
                "mousedown".to_string(),
                "resize".to_string(),
                "keydown".to_string(),
                "touchstart".to_string(),
                "wheel".to_string(),
            ],
            initial_state: false,
            timeout: 60000, // Default to 1 minute
        }
    }
}

/// Hook to track whether the user is idle
///
/// # Arguments
///
/// * `options` - Configuration options including idle timeout and events to track
///
/// # Returns
///
/// A tuple containing:
/// - A boolean indicating if the user is idle
/// - Last activity timestamp in milliseconds
/// - Function to manually set the user as idle
/// - Function to manually set the user as active
///
/// # Example
///
/// ```rust
/// use yewuse::browser::{use_idle, UseIdleOptions};
/// use yew::prelude::*;
///
/// #[function_component(IdleDetector)]
/// fn idle_detector() -> Html {
///     let options = UseIdleOptions {
///         timeout: 5000, // 5 seconds
///         ..Default::default()
///     };
///     
///     let (is_idle, last_active, set_idle, set_active) = use_idle(Some(options));
///     
///     html! {
///         <div>
///             <p>{"User is "}{if *is_idle { "idle" } else { "active" }}</p>
///             <p>{"Last activity: "}{last_active}{"ms"}</p>
///             <button onclick={move |_| set_idle.emit(())}>{"Set Idle"}</button>
///             <button onclick={move |_| set_active.emit(())}>{"Set Active"}</button>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_idle(options: Option<UseIdleOptions>) -> (
    UseStateHandle<bool>,      // is_idle
    UseStateHandle<f64>,       // last_active
    Callback<()>,              // set_idle
    Callback<()>,              // set_active
) {
    let options = options.unwrap_or_default();
    let is_idle = use_state(|| options.initial_state);
    let last_active = use_state(|| js_sys::Date::now());
    let timeout_handle = use_mut_ref(|| None::<i32>);

    // Function to reset the idle timer
    let reset_timer = {
        let is_idle = is_idle.clone();
        let last_active = last_active.clone();
        let timeout_handle = timeout_handle.clone();
        let timeout_ms = options.timeout;

        move || {
            // Update last active time
            last_active.set(js_sys::Date::now());
            
            // If user was idle, mark them as active
            if *is_idle {
                is_idle.set(false);
            }
            
            // Clear existing timeout
            if let Some(handle) = *timeout_handle.borrow() {
                window().unwrap().clear_timeout_with_handle(handle);
            }
            
            // Set new timeout
            let new_is_idle = is_idle.clone();
            let window = window().unwrap();
            
            let callback = Closure::once(Box::new(move || {
                new_is_idle.set(true);
            }) as Box<dyn FnOnce()>);
            
            let handle = window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    callback.as_ref().unchecked_ref(),
                    timeout_ms as i32,
                )
                .unwrap();
                
            // Store the new timeout handle
            *timeout_handle.borrow_mut() = Some(handle);
            
            // Forget the closure to prevent it from being dropped
            callback.forget();
        }
    };

    // Function to force set idle state
    let set_idle = {
        let is_idle = is_idle.clone();
        let timeout_handle = timeout_handle.clone();
        
        Callback::from(move |_| {
            is_idle.set(true);
            
            // Clear existing timeout
            if let Some(handle) = *timeout_handle.borrow() {
                window().unwrap().clear_timeout_with_handle(handle);
                *timeout_handle.borrow_mut() = None;
            }
        })
    };

    // Function to force set active state and reset timer
    let set_active = {
        let reset_timer = reset_timer.clone();
        
        Callback::from(move |_| {
            reset_timer();
        })
    };

    // Set up event listeners to reset timer on user activity
    {
        let events = options.events.clone();
        let reset_timer = reset_timer.clone();
        
        use_effect_with_deps(
            move |(events, _)| {
                let window = window().unwrap();
                let document = window.document().unwrap();
                let event_targets: Vec<(web_sys::EventTarget, String)> = events
                    .iter()
                    .map(|event| (window.clone().into(), event.clone()))
                    .collect();
                
                // Store all event listeners to remove them later
                let mut listeners = Vec::new();
                
                for (target, event_type) in event_targets {
                    let reset_timer = reset_timer.clone();
                    
                    let callback = Closure::wrap(Box::new(move |_| {
                        reset_timer();
                    }) as Box<dyn FnMut(_)>);
                    
                    target
                        .add_event_listener_with_callback(&event_type, callback.as_ref().unchecked_ref())
                        .unwrap();
                    
                    listeners.push((target, event_type, callback));
                }
                
                // Initial timer
                reset_timer();
                
                // Cleanup
                move || {
                    for (target, event_type, callback) in listeners {
                        target
                            .remove_event_listener_with_callback(
                                &event_type,
                                callback.as_ref().unchecked_ref(),
                            )
                            .unwrap();
                    }
                    
                    // Clear timeout on unmount
                    if let Some(handle) = *timeout_handle.borrow() {
                        window().unwrap().clear_timeout_with_handle(handle);
                    }
                }
            },
            (events, timeout_handle.clone()),
        );
    }

    (is_idle, last_active, set_idle, set_active)
}