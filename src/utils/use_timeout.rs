use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::window;
use yew::prelude::*;

/// Hook for setting up a timeout that is automatically cleared when the component unmounts
///
/// # Arguments
///
/// * `callback` - Function to call when the timeout expires
/// * `delay_ms` - Timeout delay in milliseconds. If None, the timeout is paused
///
/// # Returns
///
/// A tuple containing:
/// - A function that can be called to reset the timeout timer
/// - A function that can be called to clear the timeout
///
/// # Example
///
/// ```rust
/// use yewuse::utils::use_timeout;
/// use yew::prelude::*;
///
/// #[function_component(AutoDismiss)]
/// fn auto_dismiss() -> Html {
///     let visible = use_state(|| true);
///     
///     let hide = {
///         let visible = visible.clone();
///         Callback::from(move |_| {
///             visible.set(false);
///         })
///     };
///     
///     let show = {
///         let visible = visible.clone();
///         Callback::from(move |_| {
///             visible.set(true);
///         })
///     };
///     
///     // Automatically hide after 3 seconds if visible
///     let delay = if *visible { Some(3000) } else { None };
///     let (reset, clear) = use_timeout(hide.clone(), delay);
///     
///     html! {
///         <div>
///             if *visible {
///                 <div class="notification">
///                     <p>{"This will disappear in 3 seconds"}</p>
///                     <button onclick={clear}>{"Keep visible"}</button>
///                     <button onclick={hide}>{"Dismiss now"}</button>
///                 </div>
///             } else {
///                 <button onclick={show}>{"Show notification"}</button>
///             }
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_timeout(callback: Callback<()>, delay_ms: Option<u32>) -> (Callback<()>, Callback<()>) {
    // Reference to store the timeout ID
    let timeout_id = use_mut_ref(|| None::<i32>);
    
    // Reference to store the callback closure
    let saved_callback = use_mut_ref(|| callback.clone());
    
    // Update saved callback when it changes
    {
        let mut saved_callback = saved_callback.borrow_mut();
        *saved_callback = callback.clone();
    }
    
    // Function to clear the current timeout
    let clear_timeout = {
        let timeout_id = timeout_id.clone();
        
        Callback::from(move |_| {
            let mut id_ref = timeout_id.borrow_mut();
            if let Some(id) = *id_ref {
                window().unwrap().clear_timeout_with_handle(id);
                *id_ref = None;
            }
        })
    };
    
    // Function to set up a new timeout
    let set_timeout = {
        let timeout_id = timeout_id.clone();
        let saved_callback = saved_callback.clone();
        let clear_timeout = clear_timeout.clone();
        
        move |delay: Option<u32>| {
            // First clear any existing timeout
            clear_timeout.emit(());
            
            // If delay is Some, set up new timeout
            if let Some(delay) = delay {
                let callback = saved_callback.clone();
                
                let window = window().unwrap();
                let closure = Closure::once(Box::new(move || {
                    // Clear the timeout ID since it's now expired
                    *timeout_id.borrow_mut() = None;
                    
                    // Call the user's callback
                    callback.borrow().emit(());
                }) as Box<dyn FnOnce()>);
                
                let id = window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        closure.as_ref().unchecked_ref(),
                        delay as i32,
                    )
                    .unwrap();
                
                // Store the new timeout ID
                *timeout_id.borrow_mut() = Some(id);
                
                // Make sure the closure lives until the timeout executes
                closure.forget();
            }
        }
    };
    
    // Set up timeout when delay changes
    use_effect_with_deps(
        move |delay| {
            set_timeout(*delay);
            
            // Clean up on unmount or when delay changes
            move || {
                clear_timeout.emit(());
            }
        },
        delay_ms,
    );
    
    // Return a callback to reset and clear the timer
    let reset = Callback::from(move |_| {
        if let Some(delay) = delay_ms {
            clear_timeout.emit(());
            set_timeout(Some(delay));
        }
    });
    
    (reset, clear_timeout)
}