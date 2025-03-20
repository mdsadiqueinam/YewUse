use gloo::storage::{SessionStorage, Storage};
use serde::{de::DeserializeOwned, Serialize};
use yew::prelude::*;

/// Hook for using session storage with automatic serialization and deserialization
///
/// # Type Parameters
///
/// * `T` - The type of the value to store, must implement Serialize and DeserializeOwned
///
/// # Arguments
///
/// * `key` - The key under which to store the value in session storage
/// * `default_value` - The default value to use if no value is found in session storage
///
/// # Returns
///
/// A tuple containing the current value and a function to update the value
///
/// # Example
///
/// ```rust
/// use yewuse::browser::use_session_storage;
///
/// #[function_component(TempPreferences)]
/// fn temp_preferences() -> Html {
///     let (theme, set_theme) = use_session_storage::<String>("theme", "light".to_string());
///     
///     html! {
///         <div>
///             <p>{"Current theme: "}{theme.clone()}</p>
///             <button onclick={move |_| set_theme("dark".to_string())}>{"Switch to Dark"}</button>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_session_storage<T>(key: &str, default_value: T) -> (UseStateHandle<T>, Callback<T>)
where
    T: Clone + Serialize + DeserializeOwned + 'static,
{
    let key = key.to_owned();
    
    // Initialize state with value from sessionStorage or default
    let state = use_state(move || -> T {
        match SessionStorage::get(&key) {
            Ok(value) => value,
            Err(_) => {
                // If nothing in storage or error deserializing, save the default
                if let Ok(()) = SessionStorage::set(&key, &default_value) {
                    default_value
                } else {
                    default_value
                }
            }
        }
    });
    
    // Create a setter that updates both state and sessionStorage
    let key_setter = key.clone();
    let state_setter = state.clone();
    let setter = Callback::from(move |value: T| {
        if let Ok(()) = SessionStorage::set(&key_setter, &value) {
            state_setter.set(value);
        }
    });
    
    (state, setter)
}