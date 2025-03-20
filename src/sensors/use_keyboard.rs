use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{window, Event, KeyboardEvent};
use yew::prelude::*;
use std::collections::HashSet;

/// Configuration options for keyboard events
#[derive(Clone, PartialEq)]
pub struct UseKeyboardOptions {
    /// Whether to listen on the window (true) or the specific target element (false)
    pub target_is_window: bool,
    /// Whether to prevent the default behavior of the event
    pub prevent_default: bool,
    /// Whether to stop propagation of the event
    pub stop_propagation: bool,
    /// List of events to listen for (default: ["keydown"])
    pub events: Vec<String>,
}

impl Default for UseKeyboardOptions {
    fn default() -> Self {
        Self {
            target_is_window: true,
            prevent_default: false,
            stop_propagation: false,
            events: vec!["keydown".to_string()],
        }
    }
}

/// Hook for detecting keyboard events
///
/// # Arguments
///
/// * `key_filter` - Either a single key or list of keys to detect (use empty string to detect all keys)
/// * `handler` - Callback to execute when the key(s) are pressed
/// * `target` - Optional target element (defaults to window if None)
/// * `options` - Optional configuration for the keyboard event listener
///
/// # Example
///
/// ```rust
/// use yewuse::sensors::use_keyboard;
/// use yew::prelude::*;
///
/// #[function_component(KeyboardShortcut)]
/// fn keyboard_shortcut() -> Html {
///     let pressed_key = use_state(|| String::new());
///     
///     let on_key = {
///         let pressed_key = pressed_key.clone();
///         Callback::from(move |e: web_sys::KeyboardEvent| {
///             pressed_key.set(e.key());
///         })
///     };
///     
///     // Listen for Escape key or Space key
///     use_keyboard(vec!["Escape".to_string(), " ".to_string()], on_key, None, None);
///     
///     html! {
///         <div>
///             <p>{"Press Escape or Space to trigger the handler"}</p>
///             <p>{"Last pressed key: "}{(*pressed_key).clone()}</p>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_keyboard<F>(
    key_filter: impl Into<KeyFilter>,
    handler: Callback<KeyboardEvent>,
    target: Option<NodeRef>,
    options: Option<UseKeyboardOptions>,
) where
    F: FnOnce(KeyboardEvent) + 'static,
{
    let options = options.unwrap_or_default();
    let key_filter: KeyFilter = key_filter.into();
    let key_set = use_memo(
        |filter| match filter {
            KeyFilter::None => None,
            KeyFilter::Keys(keys) => {
                let set: HashSet<String> = keys.iter().cloned().collect();
                Some(set)
            }
        },
        key_filter,
    );

    use_effect_with_deps(
        move |(target, handler, options, key_set)| {
            let window = window().unwrap();
            let options = options.clone();
            let handler = handler.clone();
            let key_set = key_set.clone();

            // Collection of event handlers
            let mut event_listeners = Vec::new();

            for event_type in &options.events {
                let on_key = {
                    let handler = handler.clone();
                    let key_set = key_set.clone();
                    let prevent_default = options.prevent_default;
                    let stop_propagation = options.stop_propagation;

                    Closure::wrap(Box::new(move |e: KeyboardEvent| {
                        // Filter by key if a filter is set
                        if let Some(keys) = &key_set {
                            if !keys.contains(&e.key()) {
                                return;
                            }
                        }

                        if prevent_default {
                            e.prevent_default();
                        }

                        if stop_propagation {
                            e.stop_propagation();
                        }

                        handler.emit(e);
                    }) as Box<dyn FnMut(_)>)
                };

                if options.target_is_window || target.is_none() {
                    window
                        .add_event_listener_with_callback(
                            event_type,
                            on_key.as_ref().unchecked_ref(),
                        )
                        .unwrap();
                } else if let Some(target) = target {
                    if let Some(element) = target.cast::<web_sys::Element>() {
                        element
                            .add_event_listener_with_callback(
                                event_type,
                                on_key.as_ref().unchecked_ref(),
                            )
                            .unwrap();
                    }
                }

                event_listeners.push((event_type.clone(), on_key));
            }

            // Return cleanup function
            move || {
                for (event_type, listener) in event_listeners {
                    if options.target_is_window || target.is_none() {
                        window
                            .remove_event_listener_with_callback(
                                &event_type,
                                listener.as_ref().unchecked_ref(),
                            )
                            .unwrap();
                    } else if let Some(target) = target {
                        if let Some(element) = target.cast::<web_sys::Element>() {
                            element
                                .remove_event_listener_with_callback(
                                    &event_type,
                                    listener.as_ref().unchecked_ref(),
                                )
                                .unwrap();
                        }
                    }
                }
            }
        },
        (target, handler, options, key_set),
    );
}

/// Key filter for the use_keyboard hook
#[derive(Clone, PartialEq)]
pub enum KeyFilter {
    /// No filter, listen for all keys
    None,
    /// Filter for specific keys
    Keys(Vec<String>),
}

impl<T: AsRef<str>> From<Vec<T>> for KeyFilter {
    fn from(keys: Vec<T>) -> Self {
        KeyFilter::Keys(keys.iter().map(|k| k.as_ref().to_string()).collect())
    }
}

impl From<String> for KeyFilter {
    fn from(key: String) -> Self {
        if key.is_empty() {
            KeyFilter::None
        } else {
            KeyFilter::Keys(vec![key])
        }
    }
}

impl From<&str> for KeyFilter {
    fn from(key: &str) -> Self {
        if key.is_empty() {
            KeyFilter::None
        } else {
            KeyFilter::Keys(vec![key.to_string()])
        }
    }
}

/// Keyboard event information
#[derive(Clone, Debug, PartialEq)]
pub struct KeyboardState {
    /// The key that was pressed (e.g., "ArrowUp", "a", "Enter")
    pub key: String,
    /// The key code (deprecated but included for compatibility)
    pub key_code: u32,
    /// The key's location on the keyboard
    pub location: u32,
    /// Whether Ctrl was pressed
    pub ctrl_key: bool,
    /// Whether Shift was pressed
    pub shift_key: bool,
    /// Whether Alt was pressed
    pub alt_key: bool,
    /// Whether Meta (Windows/Command) was pressed
    pub meta_key: bool,
    /// Whether the key is being held down (true for keydown, false for keyup)
    pub pressed: bool,
    /// Currently pressed keys
    pub keys_pressed: HashSet<String>,
}

impl Default for KeyboardState {
    fn default() -> Self {
        Self {
            key: String::new(),
            key_code: 0,
            location: 0,
            ctrl_key: false,
            shift_key: false,
            alt_key: false,
            meta_key: false,
            pressed: false,
            keys_pressed: HashSet::new(),
        }
    }
}

/// Hook for tracking the state of all keyboard keys
///
/// # Arguments
///
/// * `target` - Optional element to attach event listeners to (uses document if None)
///
/// # Returns
///
/// Current keyboard state
///
/// # Example
///
/// ```rust
/// use yewuse::sensors::{use_keyboard_state, is_key_pressed, is_combo_pressed};
/// use yew::prelude::*;
///
/// #[function_component(KeyboardDemo)]
/// fn keyboard_demo() -> Html {
///     let keyboard = use_keyboard_state(None);
///     let keys_list = keyboard.keys_pressed
///         .iter()
///         .map(|key| key.to_string())
///         .collect::<Vec<_>>()
///         .join(", ");
///     
///     let is_arrow_up = is_key_pressed(&keyboard, "ArrowUp");
///     let is_ctrl_s = is_combo_pressed(&keyboard, &["Control", "s"]);
///     
///     html! {
///         <div>
///             <h2>{"Keyboard Input Demo"}</h2>
///             <p>{"Press any key (click on this page first)"}</p>
///             
///             <div class="key-info">
///                 <p>{"Last key: "}{&keyboard.key}</p>
///                 <p>{"Modifiers: "}
///                     {if keyboard.ctrl_key { "Ctrl " } else { "" }}
///                     {if keyboard.shift_key { "Shift " } else { "" }}
///                     {if keyboard.alt_key { "Alt " } else { "" }}
///                     {if keyboard.meta_key { "Meta " } else { "" }}
///                 </p>
///                 <p>{"Currently pressed: "}{keys_list}</p>
///                 <p>{"Arrow Up pressed: "}{is_arrow_up.to_string()}</p>
///                 <p>{"Ctrl+S pressed: "}{is_ctrl_s.to_string()}</p>
///             </div>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_keyboard_state(target: Option<NodeRef>) -> KeyboardState {
    let keyboard_state = use_state(KeyboardState::default);
    let keys_pressed = use_mut_ref(|| HashSet::<String>::new());
    
    use_effect_with_deps(
        move |(target, keyboard_state)| {
            let keyboard_state = keyboard_state.clone();
            let keys_pressed = keys_pressed.clone();
            
            let document = window().unwrap().document().unwrap();
            let target_element = if let Some(node) = target {
                if let Some(element) = node.get() {
                    element
                } else {
                    document.dyn_into().unwrap()
                }
            } else {
                document.dyn_into().unwrap()
            };
            
            // Handler for keydown event
            let keydown_callback = {
                let keyboard_state = keyboard_state.clone();
                let keys_pressed = keys_pressed.clone();
                
                Closure::wrap(Box::new(move |event: KeyboardEvent| {
                    let key = event.key();
                    
                    // Add to set of pressed keys
                    keys_pressed.borrow_mut().insert(key.clone());
                    
                    keyboard_state.set(KeyboardState {
                        key: key,
                        key_code: event.key_code(),
                        location: event.location(),
                        ctrl_key: event.ctrl_key(),
                        shift_key: event.shift_key(),
                        alt_key: event.alt_key(),
                        meta_key: event.get_modifier_state("Meta"),
                        pressed: true,
                        keys_pressed: keys_pressed.borrow().clone(),
                    });
                }) as Box<dyn FnMut(_)>)
            };
            
            // Handler for keyup event
            let keyup_callback = {
                let keyboard_state = keyboard_state.clone();
                let keys_pressed = keys_pressed.clone();
                
                Closure::wrap(Box::new(move |event: KeyboardEvent| {
                    let key = event.key();
                    
                    // Remove from set of pressed keys
                    keys_pressed.borrow_mut().remove(&key);
                    
                    keyboard_state.set(KeyboardState {
                        key: key,
                        key_code: event.key_code(),
                        location: event.location(),
                        ctrl_key: event.ctrl_key(),
                        shift_key: event.shift_key(),
                        alt_key: event.alt_key(),
                        meta_key: event.get_modifier_state("Meta"),
                        pressed: false,
                        keys_pressed: keys_pressed.borrow().clone(),
                    });
                }) as Box<dyn FnMut(_)>)
            };
            
            // Add event listeners
            target_element
                .add_event_listener_with_callback("keydown", keydown_callback.as_ref().unchecked_ref())
                .unwrap();
            
            target_element
                .add_event_listener_with_callback("keyup", keyup_callback.as_ref().unchecked_ref())
                .unwrap();
            
            // Return cleanup function
            move || {
                target_element
                    .remove_event_listener_with_callback("keydown", keydown_callback.as_ref().unchecked_ref())
                    .unwrap();
                
                target_element
                    .remove_event_listener_with_callback("keyup", keyup_callback.as_ref().unchecked_ref())
                    .unwrap();
            }
        },
        (target, keyboard_state.clone()),
    );
    
    (*keyboard_state).clone()
}

/// Check if a specific key is currently pressed
///
/// # Arguments
///
/// * `keyboard` - The keyboard state from use_keyboard_state
/// * `key` - The key to check
///
/// # Returns
///
/// True if the key is currently pressed
pub fn is_key_pressed(keyboard: &KeyboardState, key: &str) -> bool {
    keyboard.keys_pressed.contains(key)
}

/// Check if a specific combination of keys is pressed
///
/// # Arguments
///
/// * `keyboard` - The keyboard state from use_keyboard_state
/// * `keys` - The keys to check
///
/// # Returns
///
/// True if all the specified keys are currently pressed
pub fn is_combo_pressed(keyboard: &KeyboardState, keys: &[&str]) -> bool {
    keys.iter().all(|key| keyboard.keys_pressed.contains(*key))
}