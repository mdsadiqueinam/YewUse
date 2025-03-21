use wasm_bindgen::JsCast;
use web_sys::window;
use yew::prelude::*;

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
pub fn use_scroll_lock() -> (
    bool,                // is locked
    Callback<()>,        // lock function
    Callback<()>,        // unlock function
    Callback<()>,        // toggle function
) {
    let is_locked = use_state(|| false);
    let original_style = use_mut_ref(|| String::new());
    
    // Function to lock scrolling
    let lock = {
        let is_locked = is_locked.clone();
        let original_style = original_style.clone();
        
        Callback::from(move |_| {
            if *is_locked {
                return; // Already locked
            }
            
            if let Some(window) = window() {
                if let Some(document) = window.document() {
                    let body = document.body().unwrap();
                    
                    // Store original style
                    let current_style = body.get_attribute("style").unwrap_or_default();
                    *original_style.borrow_mut() = current_style;
                    
                    // Get window dimensions to set correct width (prevent layout shift)
                    let window_width = window.inner_width().unwrap().as_f64().unwrap();
                    
                    // Apply lock style
                    let lock_style = format!(
                        "{}overflow: hidden; position: fixed; top: 0; right: 0; bottom: 0; left: 0; width: {}px;",
                        if original_style.borrow().is_empty() { "" } else { &*original_style.borrow() + "; " },
                        window_width
                    );
                    
                    body.set_attribute("style", &lock_style).unwrap();
                    is_locked.set(true);
                }
            }
        })
    };
    
    // Function to unlock scrolling
    let unlock = {
        let is_locked = is_locked.clone();
        let original_style = original_style.clone();
        
        Callback::from(move |_| {
            if !*is_locked {
                return; // Already unlocked
            }
            
            if let Some(window) = window() {
                if let Some(document) = window.document() {
                    let body = document.body().unwrap();
                    
                    // Restore original style
                    if original_style.borrow().is_empty() {
                        body.remove_attribute("style").unwrap();
                    } else {
                        body.set_attribute("style", &original_style.borrow()).unwrap();
                    }
                    
                    is_locked.set(false);
                }
            }
        })
    };
    
    // Function to toggle scrolling state
    let toggle = {
        let is_locked = is_locked.clone();
        let lock = lock.clone();
        let unlock = unlock.clone();
        
        Callback::from(move |_| {
            if *is_locked {
                unlock.emit(());
            } else {
                lock.emit(());
            }
        })
    };
    
    // Clean up on unmount
    {
        let unlock = unlock.clone();
        
        use_effect_with_deps(
            move |_| {
                || {
                    // Make sure to unlock if component unmounts while locked
                    unlock.emit(());
                }
            },
            (),
        );
    }
    
    (*is_locked, lock, unlock, toggle)
}