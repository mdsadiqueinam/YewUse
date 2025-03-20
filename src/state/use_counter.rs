use std::ops::{Add, Sub};
use yew::prelude::*;

/// Actions for a counter
#[derive(Debug, Clone)]
pub struct CounterActions<T> {
    /// Increment the counter by the given value or default step
    pub increment: Callback<Option<T>>,
    /// Decrement the counter by the given value or default step
    pub decrement: Callback<Option<T>>,
    /// Reset the counter to its initial value
    pub reset: Callback<()>,
    /// Set the counter to a specific value
    pub set: Callback<T>,
}

/// Hook for managing a counter with custom increment/decrement operations
///
/// # Type Parameters
///
/// * `T` - The numeric type for the counter, must implement Add, Sub, Copy and PartialOrd
///
/// # Arguments
///
/// * `initial_value` - The initial value of the counter
/// * `options` - Optional configuration containing:
///   * `min` - Minimum value (counter won't go below this)
///   * `max` - Maximum value (counter won't go above this)
///   * `step` - Default increment/decrement step
///
/// # Returns
///
/// A tuple containing:
/// - The current counter value
/// - Actions for manipulating the counter (increment, decrement, reset, set)
///
/// # Example
///
/// ```rust
/// use yewuse::state::use_counter;
///
/// #[function_component(Counter)]
/// fn counter() -> Html {
///     let (count, actions) = use_counter(0, Some((
///         Some(-10),  // min
///         Some(10),   // max
///         Some(2),    // step
///     )));
///     
///     html! {
///         <div>
///             <p>{"Count: "}{*count}</p>
///             <button onclick={move |_| actions.decrement.emit(None)}>{"Decrement by 2"}</button>
///             <button onclick={move |_| actions.increment.emit(None)}>{"Increment by 2"}</button>
///             <button onclick={move |_| actions.increment.emit(Some(5))}>{"Increment by 5"}</button>
///             <button onclick={move |_| actions.reset.emit(())}>{"Reset"}</button>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_counter<T>(
    initial_value: T,
    options: Option<(Option<T>, Option<T>, Option<T>)>,
) -> (UseStateHandle<T>, CounterActions<T>)
where
    T: Add<Output = T> + Sub<Output = T> + Copy + PartialOrd + 'static,
{
    let (min, max, step) = if let Some((min, max, step)) = options {
        (min, max, step)
    } else {
        (None, None, None)
    };
    
    let counter = use_state(|| initial_value);
    
    // Determine the actual step value to use
    let step_value = step.unwrap_or_else(|| {
        // Default step implementation depends on the type
        // This is a simplified version - in real implementation
        // you might want to use type traits or other methods
        // to determine a sensible default step for the type
        unsafe { std::mem::transmute_copy(&1) } // This is a hack to get a "1" for any numeric type
    });
    
    // Increment action
    let counter_inc = counter.clone();
    let increment = Callback::from(move |amount: Option<T>| {
        let step_to_use = amount.unwrap_or(step_value);
        let new_value = *counter_inc + step_to_use;
        
        // Check if the new value exceeds max
        if let Some(max_val) = max {
            if new_value > max_val {
                counter_inc.set(max_val);
                return;
            }
        }
        
        counter_inc.set(new_value);
    });
    
    // Decrement action
    let counter_dec = counter.clone();
    let decrement = Callback::from(move |amount: Option<T>| {
        let step_to_use = amount.unwrap_or(step_value);
        let new_value = *counter_dec - step_to_use;
        
        // Check if the new value is below min
        if let Some(min_val) = min {
            if new_value < min_val {
                counter_dec.set(min_val);
                return;
            }
        }
        
        counter_dec.set(new_value);
    });
    
    // Reset action
    let counter_reset = counter.clone();
    let reset = Callback::from(move |_| {
        counter_reset.set(initial_value);
    });
    
    // Set action
    let counter_set = counter.clone();
    let set = Callback::from(move |value: T| {
        let mut new_value = value;
        
        // Check min/max constraints
        if let Some(min_val) = min {
            if new_value < min_val {
                new_value = min_val;
            }
        }
        
        if let Some(max_val) = max {
            if new_value > max_val {
                new_value = max_val;
            }
        }
        
        counter_set.set(new_value);
    });
    
    let actions = CounterActions {
        increment,
        decrement,
        reset,
        set,
    };
    
    (counter, actions)
}