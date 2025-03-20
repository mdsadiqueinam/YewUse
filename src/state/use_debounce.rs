use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::window;
use yew::prelude::*;

/// Hook for debouncing a value that changes frequently
///
/// # Type Parameters
///
/// * `T` - The type of the value to debounce, must implement Clone and PartialEq
///
/// # Arguments
///
/// * `value` - The value to debounce
/// * `delay_ms` - The delay in milliseconds before updating the debounced value
///
/// # Returns
///
/// The debounced value
///
/// # Example
///
/// ```rust
/// use yewuse::state::use_debounce;
/// use yew::prelude::*;
///
/// #[function_component(SearchInput)]
/// fn search_input() -> Html {
///     let input = use_state(|| String::new());
///     let debounced_input = use_debounce((*input).clone(), 500);
///     
///     // This effect will only run when the debounced value changes
///     use_effect_with_deps(|input| {
///         // Perform search with the debounced input
///         println!("Searching for: {}", input);
///         || {}
///     }, debounced_input.clone());
///     
///     let oninput = {
///         let input = input.clone();
///         Callback::from(move |e: InputEvent| {
///             let target = e.target_dyn_into::<web_sys::HtmlInputElement>();
///             if let Some(target) = target {
///                 input.set(target.value());
///             }
///         })
///     };
///     
///     html! {
///         <div>
///             <input type="text" oninput={oninput} />
///             <p>{"Input: "}{(*input).clone()}</p>
///             <p>{"Debounced: "}{debounced_input.clone()}</p>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_debounce<T>(value: T, delay_ms: u32) -> T
where
    T: Clone + PartialEq + 'static,
{
    let debounced_value = use_state(|| value.clone());
    let timeout_handle = use_ref(|| None::<i32>);
    
    use_effect_with_deps(
        move |(value, delay_ms)| {
            let value = value.clone();
            let timeout_handle_rc = timeout_handle.clone();
            
            // Clear existing timeout if any
            if let Some(handle) = *timeout_handle_rc.borrow() {
                window()
                    .unwrap()
                    .clear_timeout_with_handle(handle);
            }
            
            // Set a new timeout
            let window = window().unwrap();
            let new_value = value.clone();
            let debounced_value_setter = debounced_value.clone();
            
            let closure = Closure::once(Box::new(move || {
                debounced_value_setter.set(new_value);
            }) as Box<dyn FnOnce()>);
            
            let handle = window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    *delay_ms as i32,
                )
                .unwrap();
                
            // Store the new timeout handle
            timeout_handle_rc.replace(Some(handle));
            
            // Forget the closure to prevent it from being dropped
            closure.forget();
            
            // Cleanup function
            move || {}
        },
        (value, delay_ms),
    );
    
    (*debounced_value).clone()
}