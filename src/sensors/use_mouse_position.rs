use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{window, Element, Event, MouseEvent, TouchEvent};
use yew::prelude::*;
use crate::utils::dom_utils;

#[derive(Debug, Clone)]
pub struct MousePosition {
    pub x: i32,
    pub y: i32,
    pub source_type: EventType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    Mouse,
    Touch,
}

#[derive(Clone)]
pub struct UseMousePositionOptions {
    pub touch: bool,
    pub reset_on_touch_ends: bool,
    pub target: Option<Element>,
    pub touch_only: bool,
}

impl Default for UseMousePositionOptions {
    fn default() -> Self {
        Self {
            touch: true,
            reset_on_touch_ends: false,
            target: None,
            touch_only: false,
        }
    }
}

impl PartialEq for UseMousePositionOptions {
    fn eq(&self, other: &Self) -> bool {
        self.touch == other.touch
            && self.reset_on_touch_ends == other.reset_on_touch_ends
            && self.touch_only == other.touch_only
            && match (&self.target, &other.target) {
                (Some(a), Some(b)) => a.is_same_node(Some(b)),
                (None, None) => true,
                _ => false,
            }
    }
}

#[hook]
pub fn use_mouse_position(options: UseMousePositionOptions) -> MousePosition {
    let position = yew::use_state(|| MousePosition {
        x: 0,
        y: 0,
        source_type: EventType::Mouse,
    });

    yew::use_effect_with_deps(
        move |(options,)| {
            let position_clone = position.clone();
            let target = options.target.clone().unwrap_or_else(|| {
                window()
                    .expect("no window")
                    .document()
                    .expect("no document")
                    .document_element()
                    .expect("no document element")
            });

            let mouse_handler = move |e: MouseEvent| {
                if !options.touch_only {
                    position_clone.set(MousePosition {
                        x: e.client_x(),
                        y: e.client_y(),
                        source_type: EventType::Mouse,
                    });
                }
            };

            let touch_handler = move |e: TouchEvent| {
                if options.touch {
                    if let Some(touch) = e.touches().item(0) {
                        position_clone.set(MousePosition {
                            x: touch.client_x(),
                            y: touch.client_y(),
                            source_type: EventType::Touch,
                        });
                    }
                }
            };

            let touch_end_handler = move |_: TouchEvent| {
                if options.reset_on_touch_ends {
                    position_clone.set(MousePosition {
                        x: 0,
                        y: 0,
                        source_type: EventType::Touch,
                    });
                }
            };

            let mouse_handler = Box::new(mouse_handler) as Box<dyn FnMut(_)>;
            let touch_handler = Box::new(touch_handler) as Box<dyn FnMut(_)>;
            let touch_end_handler = Box::new(touch_end_handler) as Box<dyn FnMut(_)>;

            let mouse_closure = wasm_bindgen::closure::Closure::wrap(mouse_handler);
            let touch_closure = wasm_bindgen::closure::Closure::wrap(touch_handler);
            let touch_end_closure = wasm_bindgen::closure::Closure::wrap(touch_end_handler);

            target
                .add_event_listener_with_callback("mousemove", mouse_closure.as_ref().unchecked_ref())
                .unwrap();
            
            if options.touch {
                target
                    .add_event_listener_with_callback("touchstart", touch_closure.as_ref().unchecked_ref())
                    .unwrap();
                target
                    .add_event_listener_with_callback("touchmove", touch_closure.as_ref().unchecked_ref())
                    .unwrap();
                target
                    .add_event_listener_with_callback("touchend", touch_end_closure.as_ref().unchecked_ref())
                    .unwrap();
            }

            move || {
                target
                    .remove_event_listener_with_callback(
                        "mousemove",
                        mouse_closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();

                if options.touch {
                    target
                        .remove_event_listener_with_callback(
                            "touchstart",
                            touch_closure.as_ref().unchecked_ref(),
                        )
                        .unwrap();
                    target
                        .remove_event_listener_with_callback(
                            "touchmove",
                            touch_closure.as_ref().unchecked_ref(),
                        )
                        .unwrap();
                    target
                        .remove_event_listener_with_callback(
                            "touchend",
                            touch_end_closure.as_ref().unchecked_ref(),
                        )
                        .unwrap();
                }
            }
        },
        (options,),
    );

    (*position).clone()
}

/// Hook for tracking mouse position with a simplified API
///
/// This is a simplified version of use_mouse_position with no configuration options.
///
/// # Returns
///
/// The current mouse position coordinates
///
/// # Example
///
/// ```rust
/// use yewuse::sensors::use_mouse_position_simple;
///
/// #[function_component(MouseTracker)]
/// fn mouse_tracker() -> Html {
///     let position = use_mouse_position_simple();
///     
///     html! {
///         <div>
///             <p>{"Mouse position: "}{position.x}{", "}{position.y}</p>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_mouse_position_simple() -> MousePosition {
    // Call the full version with default options (no target element)
    use_mouse_position(UseMousePositionOptions::default())
}