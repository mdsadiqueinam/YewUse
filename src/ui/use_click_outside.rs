use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{window, Element, Event, MouseEvent};
use yew::prelude::*;

/// Hook to detect clicks outside a specified element
///
/// # Arguments
///
/// * `node_ref` - Reference to the element to monitor
/// * `callback` - Function to call when a click outside is detected
/// * `enabled` - Whether the detection is currently active (optional, defaults to true)
///
/// # Example
///
/// ```rust
/// use yewuse::ui::use_click_outside;
/// use yew::prelude::*;
///
/// #[function_component(Dropdown)]
/// fn dropdown() -> Html {
///     let dropdown_ref = use_node_ref();
///     let is_open = use_state(|| false);
///     
///     let toggle = {
///         let is_open = is_open.clone();
///         Callback::from(move |_| {
///             is_open.set(!*is_open);
///         })
///     };
///     
///     let close_dropdown = {
///         let is_open = is_open.clone();
///         Callback::from(move |_| {
///             is_open.set(false);
///         })
///     };
///     
///     // Only detect clicks outside when the dropdown is open
///     use_click_outside(dropdown_ref.clone(), close_dropdown, Some(*is_open));
///     
///     html! {
///         <div>
///             <button onclick={toggle}>{"Toggle Dropdown"}</button>
///             
///             if *is_open {
///                 <div ref={dropdown_ref} class="dropdown-content">
///                     <p>{"Click outside to close this dropdown"}</p>
///                     <a href="#">{"Option 1"}</a>
///                     <a href="#">{"Option 2"}</a>
///                     <a href="#">{"Option 3"}</a>
///                 </div>
///             }
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_click_outside(
    node_ref: NodeRef,
    callback: Callback<MouseEvent>,
    enabled: Option<bool>,
) {
    let enabled = enabled.unwrap_or(true);
    
    use_effect_with_deps(
        move |(node_ref, callback, enabled)| {
            // Skip if not enabled
            if !enabled {
                return || {};
            }
            
            let document = window()
                .expect("window is not available")
                .document()
                .expect("document is not available");
            
            let listener = {
                let node_ref = node_ref.clone();
                let callback = callback.clone();
                
                Closure::wrap(Box::new(move |event: MouseEvent| {
                    // Get the element that was referenced
                    if let Some(element) = node_ref.cast::<Element>() {
                        // Get the target element that was clicked
                        if let Some(target) = event.target() {
                            let target_element = target.dyn_into::<Element>().ok();
                            
                            // Check if the click was outside the referenced element
                            if let Some(target_element) = target_element {
                                if !element.contains(Some(&target_element)) && &element != &target_element {
                                    callback.emit(event);
                                }
                            }
                        }
                    }
                }) as Box<dyn FnMut(_)>)
            };
            
            // Add click event listener to the document
            document
                .add_event_listener_with_callback("mousedown", listener.as_ref().unchecked_ref())
                .expect("failed to add click listener");
            
            // Cleanup function to remove the event listener
            move || {
                document
                    .remove_event_listener_with_callback("mousedown", listener.as_ref().unchecked_ref())
                    .expect("failed to remove click listener");
            }
        },
        (node_ref, callback, enabled),
    );
}