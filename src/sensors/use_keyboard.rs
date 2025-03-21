use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{window, Event, KeyboardEvent, EventTarget};
use yew::prelude::*;
use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;

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
    /// Target specific key (optional)
    pub target_key: Option<String>,
    /// Target element (for use with specific element targeting)
    pub target: KeyboardTarget,
}

#[derive(Clone, PartialEq)]
pub enum KeyboardTarget {
    Window,
    Document,
    Element(web_sys::Element),
}

impl Default for UseKeyboardOptions {
    fn default() -> Self {
        Self {
            target_is_window: true,
            prevent_default: false,
            stop_propagation: false,
            events: vec!["keydown".to_string()],
            target_key: None,
            target: KeyboardTarget::Window,
        }
    }
}

/// Keyboard state information
#[derive(Clone, Debug)]
pub struct KeyboardState {
    /// Currently pressed keys
    pub keys_pressed: HashSet<String>,
    /// The last key that was pressed
    pub last_key: Option<String>,
    /// Whether Shift was pressed
    pub shift_key: bool,
    /// Whether Ctrl was pressed
    pub ctrl_key: bool,
    /// Whether Alt was pressed
    pub alt_key: bool,
    /// Whether Meta (Windows/Command) was pressed
    pub meta_key: bool,
}

impl Default for KeyboardState {
    fn default() -> Self {
        Self {
            keys_pressed: HashSet::new(),
            last_key: None,
            shift_key: false,
            ctrl_key: false,
            alt_key: false,
            meta_key: false,
        }
    }
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

/// Hook for detecting keyboard events
///
/// # Arguments
///
/// * `key_filter` - Either a single key or list of keys to detect (use empty string to detect all keys)
/// * `handler` - Callback to execute when the key(s) are pressed
/// * `target` - Optional target element (defaults to window if None)
/// * `options` - Optional configuration for the keyboard event listener
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
            // ...existing code...
        },
        (target, handler, options, key_set),
    );
}

/// Hook for tracking the state of all keyboard keys
///
/// # Arguments
///
/// * `target` - Optional element to attach event listeners to (uses document if None)
/// * `options` - Optional configuration options
///
/// # Returns
///
/// Current keyboard state
#[hook]
pub fn use_keyboard_state(target: Option<NodeRef>, options: Option<UseKeyboardOptions>) -> KeyboardState {
    let keyboard_state = use_state(KeyboardState::default);
    let keys_pressed = use_mut_ref(|| HashSet::<String>::new());
    
    {
        let state = keyboard_state.clone();
        let keys_pressed = keys_pressed.clone();
        
        use_effect_with_deps(
            move |(target, options)| {
                let window = window().unwrap();
                let document = window.document().unwrap();
                
                let target_element: EventTarget = match &options.target {
                    KeyboardTarget::Window => window.dyn_into().unwrap(),
                    KeyboardTarget::Document => document.dyn_into().unwrap(),
                    KeyboardTarget::Element(element) => element.clone().dyn_into().unwrap(),
                };

                // Handler for keydown event
                let keydown_callback = {
                    let state = state.clone();
                    let keys_pressed = keys_pressed.clone();
                    let target_key = options.target_key.clone();
                    
                    Closure::wrap(Box::new(move |event: KeyboardEvent| {
                        let key = event.key();
                        
                        // Check target key if specified
                        if let Some(target) = &target_key {
                            if &key != target {
                                return;
                            }
                        }

                        if options.prevent_default {
                            event.prevent_default();
                        }
                        if options.stop_propagation {
                            event.stop_propagation();
                        }
                        
                        // Add to set of pressed keys
                        keys_pressed.borrow_mut().insert(key.clone());
                        
                        let mut current_state = (*state).clone();
                        current_state.keys_pressed = keys_pressed.borrow().clone();
                        current_state.last_key = Some(key);
                        current_state.shift_key = event.shift_key();
                        current_state.ctrl_key = event.ctrl_key();
                        current_state.alt_key = event.alt_key();
                        current_state.meta_key = event.get_modifier_state("Meta");
                        
                        state.set(current_state);
                    }) as Box<dyn FnMut(_)>)
                };
                
                // Handler for keyup event
                let keyup_callback = {
                    let state = state.clone();
                    let keys_pressed = keys_pressed.clone();
                    let target_key = options.target_key.clone();
                    
                    Closure::wrap(Box::new(move |event: KeyboardEvent| {
                        let key = event.key();
                        
                        // Check target key if specified
                        if let Some(target) = &target_key {
                            if &key != target {
                                return;
                            }
                        }

                        if options.prevent_default {
                            event.prevent_default();
                        }
                        if options.stop_propagation {
                            event.stop_propagation();
                        }

                        // Remove from set of pressed keys
                        keys_pressed.borrow_mut().remove(&key);
                        
                        let mut current_state = (*state).clone();
                        current_state.keys_pressed = keys_pressed.borrow().clone();
                        current_state.last_key = Some(key);
                        current_state.shift_key = event.shift_key();
                        current_state.ctrl_key = event.ctrl_key();
                        current_state.alt_key = event.alt_key();
                        current_state.meta_key = event.get_modifier_state("Meta");
                        
                        state.set(current_state);
                    }) as Box<dyn FnMut(_)>)
                };
                
                // Add event listeners
                for event_type in &options.events {
                    match event_type.as_str() {
                        "keydown" => {
                            target_element
                                .add_event_listener_with_callback("keydown", keydown_callback.as_ref().unchecked_ref())
                                .unwrap();
                        }
                        "keyup" => {
                            target_element
                                .add_event_listener_with_callback("keyup", keyup_callback.as_ref().unchecked_ref())
                                .unwrap();
                        }
                        _ => {}
                    }
                }
                
                // Keep callbacks alive
                keydown_callback.forget();
                keyup_callback.forget();
                
                // Return cleanup function
                move || {
                    // Cleanup will be handled by the browser
                }
            },
            (target, options.unwrap_or_default()),
        );
    }
    
    (*keyboard_state).clone()
}

/// Hook for detecting a specific key press
///
/// # Arguments
///
/// * `key` - The key to detect
///
/// # Returns
///
/// True if the key is currently pressed
#[hook]
pub fn use_key_press(key: String) -> bool {
    let options = UseKeyboardOptions {
        target_key: Some(key),
        ..Default::default()
    };
    
    let state = use_keyboard_state(None, Some(options));
    !state.keys_pressed.is_empty()
}

/// Hook for detecting a combination of keys
///
/// # Arguments
///
/// * `keys` - The keys to detect
///
/// # Returns
///
/// True if all the specified keys are currently pressed
#[hook]
pub fn use_key_combination(keys: Vec<String>) -> bool {
    let state = use_keyboard_state(None, None);
    keys.iter().all(|key| state.keys_pressed.contains(key))
}

/// Check if a specific key is currently pressed
pub fn is_key_pressed(keyboard: &KeyboardState, key: &str) -> bool {
    keyboard.keys_pressed.contains(key)
}

/// Check if a specific combination of keys is pressed
pub fn is_combo_pressed(keyboard: &KeyboardState, keys: &[&str]) -> bool {
    keys.iter().all(|key| keyboard.keys_pressed.contains(*key))
}