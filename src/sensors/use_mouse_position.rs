use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{window, Event, MouseEvent};
use yew::prelude::*;

/// Structure to hold mouse position coordinates
#[derive(Debug, Clone, PartialEq)]
pub struct MousePosition {
    /// X coordinate of mouse position
    pub x: i32,
    /// Y coordinate of mouse position
    pub y: i32,
}

/// Hook for tracking mouse position
///
/// # Returns
///
/// The current mouse position coordinates
///
/// # Example
///
/// ```rust
/// use yewuse::sensors::use_mouse_position;
///
/// #[function_component(MouseTracker)]
/// fn mouse_tracker() -> Html {
///     let position = use_mouse_position();
///     
///     html! {
///         <div>
///             <p>{"Mouse position: "}{position.x}{", "}{position.y}</p>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_mouse_position() -> MousePosition {
    let position = use_state(|| MousePosition { x: 0, y: 0 });

    use_effect_with_deps(
        |_| {
            let position_clone = position.clone();
            let window = window().unwrap();
            let document = window.document().unwrap();

            let callback = Closure::wrap(Box::new(move |e: MouseEvent| {
                position_clone.set(MousePosition {
                    x: e.client_x(),
                    y: e.client_y(),
                });
            }) as Box<dyn FnMut(_)>);

            document
                .add_event_listener_with_callback("mousemove", callback.as_ref().unchecked_ref())
                .unwrap();

            // Return cleanup function
            move || {
                document
                    .remove_event_listener_with_callback("mousemove", callback.as_ref().unchecked_ref())
                    .unwrap();
            }
        },
        (),
    );

    (*position).clone()
}