use yew::prelude::*;

/// Hook for managing a boolean toggle state
///
/// # Arguments
///
/// * `initial_state` - The initial boolean state
///
/// # Returns
///
/// A tuple containing:
/// - The current boolean state
/// - A function to toggle the state
/// - A function to set the state to a specific value
///
/// # Example
///
/// ```rust
/// use yewuse::state::use_toggle;
///
/// #[function_component(ToggleDemo)]
/// fn toggle_demo() -> Html {
///     let (is_active, toggle, set) = use_toggle(false);
///     
///     html! {
///         <div>
///             <p>{"State: "}{if *is_active { "Active" } else { "Inactive" }}</p>
///             <button onclick={move |_| toggle.emit(())}>{"Toggle"}</button>
///             <button onclick={move |_| set.emit(true)}>{"Set Active"}</button>
///             <button onclick={move |_| set.emit(false)}>{"Set Inactive"}</button>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_toggle(initial_state: bool) -> (UseStateHandle<bool>, Callback<()>, Callback<bool>) {
    let state = use_state(|| initial_state);
    
    // Toggle function
    let state_toggle = state.clone();
    let toggle = Callback::from(move |_| {
        let current = *state_toggle;
        state_toggle.set(!current);
    });
    
    // Set function
    let state_set = state.clone();
    let set = Callback::from(move |value: bool| {
        state_set.set(value);
    });
    
    (state, toggle, set)
}