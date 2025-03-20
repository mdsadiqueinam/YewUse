use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::window;
use yew::prelude::*;

/// Hook for setting up an interval that is automatically cleared when the component unmounts
///
/// # Arguments
///
/// * `callback` - Function to call on each interval
/// * `delay_ms` - Interval delay in milliseconds. If None, the interval is paused
///
/// # Returns
///
/// A function that can be called to reset the interval timer
///
/// # Example
///
/// ```rust
/// use yewuse::utils::use_interval;
/// use yew::prelude::*;
///
/// #[function_component(Timer)]
/// fn timer() -> Html {
///     let counter = use_state(|| 0);
///     let is_running = use_state(|| true);
///     
///     let callback = {
///         let counter = counter.clone();
///         Callback::from(move |_| {
///             counter.set(*counter + 1);
///         })
///     };
///     
///     // Only run the interval when is_running is true
///     let delay = if *is_running { Some(1000) } else { None };
///     let reset = use_interval(callback, delay);
///     
///     let toggle = {
///         let is_running = is_running.clone();
///         Callback::from(move |_| {
///             is_running.set(!*is_running);
///         })
///     };
///     
///     let reset_counter = {
///         let counter = counter.clone();
///         Callback::from(move |_| {
///             counter.set(0);
///             reset();
///         })
///     };
///     
///     html! {
///         <div>
///             <p>{"Counter: "}{*counter}</p>
///             <button onclick={toggle}>
///                 {if *is_running { "Pause" } else { "Resume" }}
///             </button>
///             <button onclick={reset_counter}>{"Reset"}</button>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_interval(callback: Callback<()>, delay_ms: Option<u32>) -> Callback<()> {
    // Reference to store the interval ID
    let interval_id = use_mut_ref(|| None::<i32>);
    
    // Reference to store the callback closure
    let saved_callback = use_mut_ref(|| callback.clone());
    
    // Update saved callback when it changes
    {
        let mut saved_callback = saved_callback.borrow_mut();
        *saved_callback = callback.clone();
    }
    
    // Function to clear the current interval
    let clear_interval = {
        let interval_id = interval_id.clone();
        
        Callback::from(move |_| {
            let mut id_ref = interval_id.borrow_mut();
            if let Some(id) = *id_ref {
                window().unwrap().clear_interval_with_handle(id);
                *id_ref = None;
            }
        })
    };
    
    // Function to set up a new interval
    let set_interval = {
        let interval_id = interval_id.clone();
        let saved_callback = saved_callback.clone();
        let clear_interval = clear_interval.clone();
        
        move |delay: Option<u32>| {
            // First clear any existing interval
            clear_interval.emit(());
            
            // If delay is Some, set up new interval
            if let Some(delay) = delay {
                let callback = saved_callback.clone();
                
                let window = window().unwrap();
                let closure = Closure::wrap(Box::new(move || {
                    callback.borrow().emit(());
                }) as Box<dyn FnMut()>);
                
                let id = window
                    .set_interval_with_callback_and_timeout_and_arguments_0(
                        closure.as_ref().unchecked_ref(),
                        delay as i32,
                    )
                    .unwrap();
                
                // Store the new interval ID
                *interval_id.borrow_mut() = Some(id);
                
                // Keep the closure alive for as long as the interval runs
                closure.forget();
            }
        }
    };
    
    // Set up interval when delay changes
    use_effect_with_deps(
        move |delay| {
            set_interval(*delay);
            
            // Clean up on unmount or when delay changes
            move || {
                clear_interval.emit(());
            }
        },
        delay_ms,
    );
    
    // Return a callback to reset the timer
    Callback::from(move |_| {
        if let Some(delay) = delay_ms {
            clear_interval.emit(());
            set_interval(Some(delay));
        }
    })
}