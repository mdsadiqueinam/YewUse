use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{window, Element, Event, MouseEvent, TouchEvent};
use yew::prelude::*;
use crate::utils::dom_utils;

/// Structure to hold mouse position coordinates
#[derive(Debug, Clone, PartialEq)]
pub struct MousePosition {
    /// X coordinate of mouse position relative to viewport
    pub x: i32,
    /// Y coordinate of mouse position relative to viewport
    pub y: i32,
    /// X coordinate of mouse position relative to the target element (if any)
    pub element_x: Option<i32>,
    /// Y coordinate of mouse position relative to the target element (if any)
    pub element_y: Option<i32>,
    /// X coordinate of mouse position relative to the document
    pub page_x: i32,
    /// Y coordinate of mouse position relative to the document
    pub page_y: i32,
}

impl Default for MousePosition {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            element_x: None,
            element_y: None,
            page_x: 0,
            page_y: 0,
        }
    }
}

/// Configuration options for mouse position tracking
#[derive(Clone, PartialEq)]
pub struct UseMousePositionOptions {
    /// If true, also track touch events (useful for mobile devices)
    pub touch_enabled: bool,
    /// If true, call event.preventDefault() on all tracked events
    pub prevent_default: bool,
    /// If set, track coordinates relative to this element instead of window
    pub relative_to_element: bool,
    /// Skip updates if mouse hasn't moved by this threshold (in pixels)
    pub update_threshold: Option<i32>,
}

impl Default for UseMousePositionOptions {
    fn default() -> Self {
        Self {
            touch_enabled: true,
            prevent_default: false,
            relative_to_element: false,
            update_threshold: None,
        }
    }
}

/// Hook for tracking mouse position
///
/// # Arguments
///
/// * `target` - Optional target element to measure position against
/// * `options` - Optional configuration options
///
/// # Returns
///
/// The current mouse position coordinates
///
/// # Example
///
/// ```rust
/// use yewuse::sensors::{use_mouse_position, UseMousePositionOptions};
/// use yew::prelude::*;
///
/// #[function_component(MouseTracker)]
/// fn mouse_tracker() -> Html {
///     let element_ref = use_node_ref();
///     let options = UseMousePositionOptions {
///         touch_enabled: true,
///         relative_to_element: true,
///         ..Default::default()
///     };
///     
///     let position = use_mouse_position(Some(element_ref.clone()), Some(options));
///     
///     html! {
///         <div ref={element_ref} style="width: 300px; height: 300px; border: 1px solid black;">
///             <p>{"Viewport coordinates: "}{position.x}{", "}{position.y}</p>
///             if let (Some(ex), Some(ey)) = (position.element_x, position.element_y) {
///                 <p>{"Element coordinates: "}{ex}{", "}{ey}</p>
///             }
///             <p>{"Page coordinates: "}{position.page_x}{", "}{position.page_y}</p>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_mouse_position(
    target: Option<NodeRef>,
    options: Option<UseMousePositionOptions>,
) -> MousePosition {
    let position = use_state(MousePosition::default);
    let last_position = use_mut_ref(MousePosition::default);
    let options = options.unwrap_or_default();

    use_effect_with_deps(
        move |(target, position, options)| {
            let position_clone = position.clone();
            let window = window().unwrap();
            let document = window.document().unwrap();
            let target_element = target.as_ref().and_then(|t| t.cast::<Element>());
            
            // Track which element to attach listeners to
            let event_target: web_sys::EventTarget = if options.relative_to_element && target_element.is_some() {
                target_element.clone().unwrap().into()
            } else {
                document.into()
            };

            // Calculate if we should update based on threshold
            let should_update = |new_x: i32, new_y: i32| -> bool {
                if let Some(threshold) = options.update_threshold {
                    let curr = last_position.borrow();
                    let dx = (new_x - curr.x).abs();
                    let dy = (new_y - curr.y).abs();
                    dx > threshold || dy > threshold
                } else {
                    true
                }
            };

            // Mouse event handler
            let mouse_callback = {
                let position_clone = position_clone.clone();
                let target_element = target_element.clone();
                let prevent_default = options.prevent_default;
                let relative_to_element = options.relative_to_element;
                let last_position = last_position.clone();

                Closure::wrap(Box::new(move |e: MouseEvent| {
                    if prevent_default {
                        e.prevent_default();
                    }

                    let client_x = e.client_x();
                    let client_y = e.client_y();

                    // Only update if we've moved more than the threshold
                    if !should_update(client_x, client_y) {
                        return;
                    }

                    let page_x = e.page_x();
                    let page_y = e.page_y();

                    // Calculate element-relative coordinates if requested
                    let (element_x, element_y) = if relative_to_element && target_element.is_some() {
                        let element = target_element.as_ref().unwrap();
                        let rect = element.get_bounding_client_rect();
                        let x = client_x - rect.left() as i32;
                        let y = client_y - rect.top() as i32;
                        (Some(x), Some(y))
                    } else {
                        (None, None)
                    };

                    let new_position = MousePosition {
                        x: client_x,
                        y: client_y,
                        element_x,
                        element_y,
                        page_x,
                        page_y,
                    };

                    // Update last known position
                    *last_position.borrow_mut() = new_position.clone();
                    position_clone.set(new_position);
                }) as Box<dyn FnMut(_)>)
            };

            // Touch event handler (for mobile devices)
            let touch_callback = if options.touch_enabled {
                let position_clone = position_clone.clone();
                let target_element = target_element.clone();
                let prevent_default = options.prevent_default;
                let relative_to_element = options.relative_to_element;
                let last_position = last_position.clone();

                let callback = Closure::wrap(Box::new(move |e: TouchEvent| {
                    if prevent_default {
                        e.prevent_default();
                    }

                    if let Some(touch) = e.touches().get(0) {
                        let client_x = touch.client_x();
                        let client_y = touch.client_y();

                        // Only update if we've moved more than the threshold
                        if !should_update(client_x, client_y) {
                            return;
                        }

                        let page_x = touch.page_x();
                        let page_y = touch.page_y();

                        // Calculate element-relative coordinates if requested
                        let (element_x, element_y) = if relative_to_element && target_element.is_some() {
                            let element = target_element.as_ref().unwrap();
                            let rect = element.get_bounding_client_rect();
                            let x = client_x - rect.left() as i32;
                            let y = client_y - rect.top() as i32;
                            (Some(x), Some(y))
                        } else {
                            (None, None)
                        };

                        let new_position = MousePosition {
                            x: client_x,
                            y: client_y,
                            element_x,
                            element_y,
                            page_x,
                            page_y,
                        };

                        // Update last known position
                        *last_position.borrow_mut() = new_position.clone();
                        position_clone.set(new_position);
                    }
                }) as Box<dyn FnMut(_)>);

                Some(callback)
            } else {
                None
            };

            // Add event listeners
            event_target
                .add_event_listener_with_callback("mousemove", mouse_callback.as_ref().unchecked_ref())
                .unwrap();

            // Add touch event listener if enabled
            if let Some(ref touch_callback) = touch_callback {
                event_target
                    .add_event_listener_with_callback("touchmove", touch_callback.as_ref().unchecked_ref())
                    .unwrap();
            }

            // Return cleanup function
            move || {
                event_target
                    .remove_event_listener_with_callback("mousemove", mouse_callback.as_ref().unchecked_ref())
                    .unwrap();

                if let Some(touch_callback) = touch_callback {
                    event_target
                        .remove_event_listener_with_callback("touchmove", touch_callback.as_ref().unchecked_ref())
                        .unwrap();
                }
            }
        },
        (target, position.clone(), options),
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
    use_mouse_position(None, None)
}