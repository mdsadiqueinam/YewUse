use std::rc::Rc;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{window, Event};
use yew::prelude::*;

/// State for the window size hook
pub struct WindowSize {
    /// Window width in pixels
    pub width: i32,
    /// Window height in pixels
    pub height: i32,
}

/// Hook for tracking window size
///
/// # Returns
///
/// A tuple containing the current window width and height
#[hook]
pub fn use_window_size() -> WindowSize {
    let size = use_state(|| {
        let window = window().unwrap();
        WindowSize {
            width: window.inner_width().unwrap().as_f64().unwrap() as i32,
            height: window.inner_height().unwrap().as_f64().unwrap() as i32,
        }
    });

    use_effect_with_deps(
        |_| {
            let size_clone = size.clone();
            let window = window().unwrap();

            let callback = Closure::wrap(Box::new(move |_e: Event| {
                let w = window.inner_width().unwrap().as_f64().unwrap() as i32;
                let h = window.inner_height().unwrap().as_f64().unwrap() as i32;
                size_clone.set(WindowSize {
                    width: w,
                    height: h,
                });
            }) as Box<dyn FnMut(_)>);

            window
                .add_event_listener_with_callback("resize", callback.as_ref().unchecked_ref())
                .unwrap();

            // Return cleanup function
            move || {
                window
                    .remove_event_listener_with_callback("resize", callback.as_ref().unchecked_ref())
                    .unwrap();
            }
        },
        (),
    );

    (*size).clone()
}