use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{window, Event, EventTarget};
use yew::prelude::*;

/// Configuration options for event listeners
#[derive(Clone, PartialEq)]
pub struct UseEventOptions {
    /// Whether to run the event listener during the capture phase
    pub capture: bool,
    /// Whether to listen only once
    pub once: bool,
    /// Whether to prevent default behavior
    pub prevent_default: bool,
    /// Whether to stop propagation
    pub stop_propagation: bool,
    /// Whether events fire when within shadow DOM
    pub composed: bool,
}

impl Default for UseEventOptions {
    fn default() -> Self {
        Self {
            capture: false,
            once: false,
            prevent_default: false,
            stop_propagation: false,
            composed: false,
        }
    }
}

/// Hook for adding event listeners with automatic cleanup
///
/// # Arguments
///
/// * `target` - Event target (element or window)
/// * `event_type` - Name of the event to listen for
/// * `callback` - Function to call when the event occurs
/// * `options` - Optional configuration for the event listener
///
/// # Example
///
/// ```rust
/// use yewuse::utils::use_event_listener;
/// use yew::prelude::*;
/// use web_sys::MouseEvent;
///
/// #[function_component(ClickTracker)]
/// fn click_tracker() -> Html {
///     let click_count = use_state(|| 0);
///     let button_ref = use_node_ref();
///     
///     let on_click = {
///         let click_count = click_count.clone();
///         Callback::from(move |_: MouseEvent| {
///             click_count.set(*click_count + 1);
///         })
///     };
///     
///     // Add a click event listener to a specific button
///     use_event_listener(
///         button_ref.clone(),
///         "click".to_string(),
///         on_click,
///         None
///     );
///     
///     html! {
///         <div>
///             <button ref={button_ref}>{"Click me"}</button>
///             <p>{"Clicks: "}{*click_count}</p>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_event_listener<T, E>(
    target: T,
    event_type: String,
    callback: Callback<E>,
    options: Option<UseEventOptions>,
) where
    T: Into<EventTargetValue> + Clone + 'static,
    E: From<Event> + 'static,
{
    let options = options.unwrap_or_default();
    
    use_effect_with_deps(
        move |(target, event_type, callback, options)| {
            let target_value = target.clone().into();
            let event_type = event_type.clone();
            let options = options.clone();
            
            // Get the actual event target
            let target_ref = match target_value {
                EventTargetValue::Window => window().unwrap().unchecked_into::<EventTarget>(),
                EventTargetValue::Document => window().unwrap().document().unwrap().unchecked_into::<EventTarget>(),
                EventTargetValue::NodeRef(node_ref) => {
                    if let Some(element) = node_ref.get() {
                        element.unchecked_into::<EventTarget>()
                    } else {
                        return || {};
                    }
                },
                EventTargetValue::EventTarget(target) => target,
            };
            
            // Create add event options
            let mut add_options = web_sys::AddEventListenerOptions::new();
            add_options.capture(options.capture);
            add_options.once(options.once);
            add_options.passive(!options.prevent_default);
            
            // Create the event listener
            let listener = {
                let callback = callback.clone();
                let prevent_default = options.prevent_default;
                let stop_propagation = options.stop_propagation;
                
                Closure::wrap(Box::new(move |event: Event| {
                    if prevent_default {
                        event.prevent_default();
                    }
                    
                    if stop_propagation {
                        event.stop_propagation();
                    }
                    
                    callback.emit(event.into());
                }) as Box<dyn FnMut(_)>)
            };
            
            // Add the event listener
            target_ref
                .add_event_listener_with_callback_and_add_event_listener_options(
                    &event_type,
                    listener.as_ref().unchecked_ref(),
                    &add_options,
                )
                .unwrap_or_else(|_| web_sys::console::error_1(&"Failed to add event listener".into()));
            
            // Return cleanup function
            move || {
                let remove_options = web_sys::EventListenerOptions::new();
                remove_options.capture(options.capture);
                
                let _ = target_ref.remove_event_listener_with_callback_and_event_listener_options(
                    &event_type,
                    listener.as_ref().unchecked_ref(),
                    &remove_options,
                );
            }
        },
        (target, event_type, callback, options),
    );
}

/// Value types that can be used as event targets
#[derive(Clone)]
pub enum EventTargetValue {
    /// Browser window
    Window,
    /// Document object
    Document,
    /// Reference to a DOM node
    NodeRef(NodeRef),
    /// Generic event target
    EventTarget(EventTarget),
}

impl From<&Window> for EventTargetValue {
    fn from(_: &Window) -> Self {
        EventTargetValue::Window
    }
}

impl From<Window> for EventTargetValue {
    fn from(_: Window) -> Self {
        EventTargetValue::Window
    }
}

impl From<NodeRef> for EventTargetValue {
    fn from(node_ref: NodeRef) -> Self {
        EventTargetValue::NodeRef(node_ref)
    }
}

impl From<EventTarget> for EventTargetValue {
    fn from(target: EventTarget) -> Self {
        EventTargetValue::EventTarget(target)
    }
}

// Allow passing None to use the window as default
impl From<Option<NodeRef>> for EventTargetValue {
    fn from(maybe_node: Option<NodeRef>) -> Self {
        match maybe_node {
            Some(node) => EventTargetValue::NodeRef(node),
            None => EventTargetValue::Window,
        }
    }
}