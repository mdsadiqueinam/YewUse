use wasm_bindgen::JsCast;
use web_sys::window;
use yew::prelude::*;
use yew::hook;

/// Hook for preventing scrolling on the document body
///
/// # Returns
///
/// A tuple containing:
/// - Whether scroll is currently locked
/// - Function to lock scrolling
/// - Function to unlock scrolling
/// - Function to toggle scrolling state
///
/// # Example
///
/// ```rust
/// use yewuse::ui::use_scroll_lock;
/// use yew::prelude::*;
///
/// #[function_component(Modal)]
/// fn modal() -> Html {
///     let (is_open, set_open, _) = use_state_with_tagger(|| false);
///     let (is_locked, lock, unlock, _) = use_scroll_lock();
///     
///     // Lock/unlock scroll when modal opens/closes
///     {
///         let is_open = *is_open;
///         let lock = lock.clone();
///         let unlock = unlock.clone();
///         
///         use_effect_with_deps(
///             move |is_open| {
///                 if *is_open {
///                     lock.emit(());
///                 } else {
///                     unlock.emit(());
///                 }
///                 || {}
///             },
///             is_open,
///         );
///     }
///     
///     let open_modal = {
///         let set_open = set_open.clone();
///         Callback::from(move |_| set_open(true))
///     };
///     
///     let close_modal = {
///         let set_open = set_open.clone();
///         Callback::from(move |_| set_open(false))
///     };
///     
///     html! {
///         <>
///             <button onclick={open_modal}>{"Open Modal"}</button>
///             
///             if *is_open {
///                 <div class="modal-overlay">
///                     <div class="modal-content">
///                         <h2>{"Modal Title"}</h2>
///                         <p>{"Scrolling is currently locked: "}{is_locked.to_string()}</p>
///                         <button onclick={close_modal}>{"Close"}</button>
///                     </div>
///                 </div>
///             }
///         </>
///     }
/// }
/// ```
#[hook]
pub fn use_scroll_lock(is_locked: bool) {
    let original_overflow = yew::use_state(String::new);
    let original_padding_right = yew::use_state(String::new);

    yew::use_effect_with_deps(
        move |(is_locked,)| {
            if let Some(window) = window() {
                if let Some(document) = window.document() {
                    if let Some(body) = document.body() {
                        if *is_locked {
                            // Store original values
                            if let Ok(style) = window.get_computed_style(&body) {
                                if let Ok(overflow) = style.get_property_value("overflow") {
                                    original_overflow.set(overflow);
                                }
                                if let Ok(padding) = style.get_property_value("padding-right") {
                                    original_padding_right.set(padding);
                                }
                            }

                            // Calculate scroll bar width
                            let window_width = window.inner_width().unwrap().as_f64().unwrap();
                            let doc_width = document.document_element().unwrap().client_width() as f64;
                            let scroll_bar_width = window_width - doc_width;

                            // Apply scroll lock
                            let style = body.style();
                            let _ = style.set_property("overflow", "hidden");
                            let _ = style.set_property(
                                "padding-right", 
                                &format!("{}px", scroll_bar_width)
                            );
                        } else {
                            // Restore original values
                            let style = body.style();
                            let _ = style.set_property("overflow", &*original_overflow);
                            let _ = style.set_property("padding-right", &*original_padding_right);
                        }
                    }
                }
            }

            // Cleanup function
            move || {
                if let Some(window) = window() {
                    if let Some(document) = window.document() {
                        if let Some(body) = document.body() {
                            let style = body.style();
                            let _ = style.set_property("overflow", &*original_overflow);
                            let _ = style.set_property("padding-right", &*original_padding_right);
                        }
                    }
                }
            }
        },
        (is_locked,),
    );
}