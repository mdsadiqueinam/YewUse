use gloo::storage::{LocalStorage, Storage};
use serde::{de::DeserializeOwned, Serialize};
use std::rc::Rc;
use yew::prelude::*;

/// Hook for using local storage with automatic serialization and deserialization
///
/// # Type Parameters
///
/// * `T` - The type of the value to store, must implement Serialize and DeserializeOwned
///
/// # Arguments
///
/// * `key` - The key under which to store the value in local storage
/// * `default_value` - The default value to use if no value is found in local storage
///
/// # Returns
///
/// A tuple containing the current value and a function to update the value
///
/// # Example
///
/// ```rust
/// use yewuse::browser::use_local_storage;
///
/// #[function_component(Counter)]
/// fn counter() -> Html {
///     let (count, set_count) = use_local_storage::<u32>("counter", 0);
///     
///     html! {
///         <div>
///             <p>{"Count: "}{*count}</p>
///             <button onclick={move |_| set_count(*count + 1)}>{"Increment"}</button>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_local_storage<T>(key: &str, default_value: T) -> (UseStateHandle<T>, Callback<T>)
where
    T: Clone + Serialize + DeserializeOwned + 'static,
{
    let key = key.to_owned();
    
    // Initialize state with value from localStorage or default
    let state = use_state(move || -> T {
        match LocalStorage::get(&key) {
            Ok(value) => value,
            Err(_) => {
                // If nothing in storage or error deserializing, save the default
                if let Ok(()) = LocalStorage::set(&key, &default_value) {
                    default_value
                } else {
                    default_value
                }
            }
        }
    });
    
    // Create a setter that updates both state and localStorage
    let key_setter = key.clone();
    let state_setter = state.clone();
    let setter = Callback::from(move |value: T| {
        if let Ok(()) = LocalStorage::set(&key_setter, &value) {
            state_setter.set(value);
        }
    });
    
    (state, setter)
}