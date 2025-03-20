use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

/// Represents the state of an asynchronous operation
#[derive(Debug, Clone, PartialEq)]
pub enum AsyncState<T, E> {
    /// The operation has not started yet
    Idle,
    /// The operation is in progress
    Loading,
    /// The operation completed successfully with a result
    Success(T),
    /// The operation failed with an error
    Error(E),
}

impl<T, E> Default for AsyncState<T, E> {
    fn default() -> Self {
        Self::Idle
    }
}

impl<T, E> AsyncState<T, E> {
    /// Returns true if the state is Idle
    pub fn is_idle(&self) -> bool {
        matches!(self, Self::Idle)
    }

    /// Returns true if the state is Loading
    pub fn is_loading(&self) -> bool {
        matches!(self, Self::Loading)
    }

    /// Returns true if the state is Success
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success(_))
    }

    /// Returns true if the state is Error
    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error(_))
    }

    /// Returns the success value if present
    pub fn value(&self) -> Option<&T> {
        match self {
            Self::Success(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the error value if present
    pub fn error(&self) -> Option<&E> {
        match self {
            Self::Error(err) => Some(err),
            _ => None,
        }
    }
}

/// Actions for controlling an asynchronous operation
#[derive(Clone)]
pub struct AsyncActions<T, E, Fn> {
    /// Executes the async function
    pub execute: Callback<()>,
    /// Resets the state back to Idle
    pub reset: Callback<()>,
    /// The current state of the async operation
    pub state: AsyncState<T, E>,
    /// The original function, in case you need to use it directly
    pub fn_ref: Rc<Fn>,
}

/// Hook for handling asynchronous operations
///
/// This hook helps manage the state of asynchronous operations in Yew components,
/// handling loading, success, and error states automatically.
///
/// # Type Parameters
///
/// * `T` - The type of the successful result
/// * `E` - The type of the error
/// * `F` - The type of the Future returned by the async function
/// * `Fn` - The type of the async function
///
/// # Arguments
///
/// * `async_fn` - The async function to execute
/// * `auto_execute` - Whether to execute the function automatically on mount
///
/// # Returns
///
/// Actions for controlling the async operation and its current state
///
/// # Example
///
/// ```rust
/// use yewuse::utils::use_async;
/// use wasm_bindgen_futures::JsFuture;
/// use wasm_bindgen::JsValue;
/// use web_sys::window;
/// use yew::prelude::*;
///
/// // A simple async function that fetches data from an API
/// async fn fetch_data() -> Result<String, JsValue> {
///     let window = window().unwrap();
///     let response = JsFuture(window.fetch_with_str("https://api.example.com/data")).await?;
///     let response = response.dyn_into::<web_sys::Response>().unwrap();
///     let text = JsFuture(response.text()?).await?;
///     Ok(text.as_string().unwrap())
/// }
///
/// #[function_component(DataFetcher)]
/// fn data_fetcher() -> Html {
///     let async_data = use_async(fetch_data, false);
///     
///     html! {
///         <div>
///             <button 
///                 onclick={async_data.execute.clone()} 
///                 disabled={async_data.state.is_loading()}
///             >
///                 {"Fetch Data"}
///             </button>
///             
///             {match &async_data.state {
///                 AsyncState::Idle => html! { <p>{"Click the button to fetch data"}</p> },
///                 AsyncState::Loading => html! { <p>{"Loading..."}</p> },
///                 AsyncState::Success(data) => html! { <p>{"Data: "}{data}</p> },
///                 AsyncState::Error(error) => html! { <p>{"Error: "}{format!("{:?}", error)}</p> },
///             }}
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_async<T, E, F, Fn>(async_fn: Fn, auto_execute: bool) -> AsyncActions<T, E, Fn>
where
    T: 'static,
    E: 'static,
    F: Future<Output = Result<T, E>> + 'static,
    Fn: FnOnce() -> F + Clone + 'static,
{
    let state = use_state(|| AsyncState::<T, E>::Idle);
    let fn_ref = use_memo(|_| Rc::new(async_fn.clone()), ());
    
    // Execute function
    let execute = {
        let state = state.clone();
        let fn_ref = fn_ref.clone();
        
        Callback::from(move |_| {
            state.set(AsyncState::Loading);
            
            let async_fn = fn_ref.as_ref().clone();
            let state = state.clone();
            
            spawn_local(async move {
                match async_fn().await {
                    Ok(data) => state.set(AsyncState::Success(data)),
                    Err(error) => state.set(AsyncState::Error(error)),
                }
            });
        })
    };
    
    // Reset state
    let reset = {
        let state = state.clone();
        
        Callback::from(move |_| {
            state.set(AsyncState::Idle);
        })
    };
    
    // Auto-execute if requested
    use_effect_with_deps(
        move |should_execute| {
            if *should_execute {
                execute.emit(());
            }
            || {}
        },
        auto_execute,
    );
    
    AsyncActions {
        execute,
        reset,
        state: (*state).clone(),
        fn_ref: fn_ref.clone(),
    }
}