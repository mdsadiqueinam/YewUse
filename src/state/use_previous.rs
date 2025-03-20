use std::rc::Rc;
use yew::prelude::*;

/// Hook for tracking the previous value of a state
///
/// # Type Parameters
///
/// * `T` - The type of the value to track
///
/// # Arguments
///
/// * `value` - The current value to track
///
/// # Returns
///
/// The previous value, or None if this is the first render
///
/// # Example
///
/// ```rust
/// use yewuse::state::use_previous;
/// use yew::prelude::*;
///
/// #[function_component(PreviousDemo)]
/// fn previous_demo() -> Html {
///     let counter = use_state(|| 0);
///     let previous = use_previous(*counter);
///     
///     html! {
///         <div>
///             <p>{"Current: "}{*counter}</p>
///             <p>{"Previous: "}{previous.unwrap_or(-1)}</p>
///             <button onclick={move |_| counter.set(*counter + 1)}>{"Increment"}</button>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_previous<T: Clone + 'static>(value: T) -> Option<T> {
    let previous = use_ref(|| None::<T>);
    let current = use_memo(|_| value.clone(), value.clone());
    
    {
        let previous = previous.clone();
        let current_value = (*current).clone();
        
        use_effect_with_deps(
            move |_| {
                previous.set(Some(current_value));
                || {}
            },
            current,
        );
    }
    
    (*previous).borrow().clone()
}