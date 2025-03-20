use std::rc::Rc;
use yew::prelude::*;

/// Hook for managing local state with reducer pattern
///
/// This hook is similar to React's useReducer, providing a more structured
/// way to manage component state with actions and a reducer function.
///
/// # Type Parameters
///
/// * `S` - The state type
/// * `A` - The action type
///
/// # Arguments
///
/// * `reducer` - A function that takes the current state and an action, and returns the new state
/// * `initial_state` - The initial state value
///
/// # Returns
///
/// A tuple containing:
/// - The current state
/// - A dispatch function that accepts actions to update the state
///
/// # Example
///
/// ```rust
/// use yewuse::state::use_local_state;
/// use yew::prelude::*;
///
/// // Define the state type
/// #[derive(Clone, PartialEq)]
/// struct TodoState {
///     items: Vec<String>,
///     input_value: String,
/// }
///
/// // Define action types
/// enum TodoAction {
///     AddItem,
///     RemoveItem(usize),
///     UpdateInput(String),
/// }
///
/// // Define the reducer function
/// fn todo_reducer(state: Rc<TodoState>, action: TodoAction) -> Rc<TodoState> {
///     match action {
///         TodoAction::AddItem => {
///             if state.input_value.trim().is_empty() {
///                 return state;
///             }
///             let mut new_items = state.items.clone();
///             new_items.push(state.input_value.clone());
///             Rc::new(TodoState {
///                 items: new_items,
///                 input_value: String::new(),
///             })
///         }
///         TodoAction::RemoveItem(index) => {
///             let mut new_items = state.items.clone();
///             if index < new_items.len() {
///                 new_items.remove(index);
///             }
///             Rc::new(TodoState {
///                 items: new_items,
///                 input_value: state.input_value.clone(),
///             })
///         }
///         TodoAction::UpdateInput(value) => Rc::new(TodoState {
///             items: state.items.clone(),
///             input_value: value,
///         }),
///     }
/// }
///
/// #[function_component(TodoApp)]
/// fn todo_app() -> Html {
///     let initial_state = Rc::new(TodoState {
///         items: Vec::new(),
///         input_value: String::new(),
///     });
///     
///     let (state, dispatch) = use_local_state(todo_reducer, initial_state);
///     
///     let on_input = {
///         let dispatch = dispatch.clone();
///         Callback::from(move |e: InputEvent| {
///             if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
///                 dispatch(TodoAction::UpdateInput(input.value()));
///             }
///         })
///     };
///     
///     let on_add = {
///         let dispatch = dispatch.clone();
///         Callback::from(move |_| dispatch(TodoAction::AddItem))
///     };
///     
///     html! {
///         <div>
///             <div>
///                 <input 
///                     type="text" 
///                     value={state.input_value.clone()} 
///                     oninput={on_input} 
///                 />
///                 <button onclick={on_add}>{"Add"}</button>
///             </div>
///             <ul>
///                 {state.items.iter().enumerate().map(|(i, item)| {
///                     let dispatch = dispatch.clone();
///                     let index = i;
///                     let remove = Callback::from(move |_| {
///                         dispatch(TodoAction::RemoveItem(index));
///                     });
///                     
///                     html! {
///                         <li key={i}>
///                             {item}
///                             <button onclick={remove}>{"Remove"}</button>
///                         </li>
///                     }
///                 }).collect::<Html>()}
///             </ul>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_local_state<S, A, F>(reducer: F, initial_state: Rc<S>) -> (Rc<S>, Callback<A>)
where
    S: PartialEq + 'static,
    A: 'static,
    F: Fn(Rc<S>, A) -> Rc<S> + 'static,
{
    let state = use_state(|| initial_state);
    
    // Create the dispatch function
    let dispatch = {
        let state = state.clone();
        Callback::from(move |action: A| {
            let current_state = (*state).clone();
            let next_state = reducer(current_state, action);
            state.set(next_state);
        })
    };
    
    ((*state).clone(), dispatch)
}