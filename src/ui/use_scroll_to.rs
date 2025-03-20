use wasm_bindgen::JsCast;
use web_sys::{window, Element, ScrollIntoViewOptions, ScrollBehavior};
use yew::prelude::*;

/// Options for scrolling to a target
#[derive(Clone, Debug)]
pub struct ScrollToOptions {
    /// Behavior of the scroll animation
    pub behavior: ScrollBehavior,
    /// Where to align the element vertically
    pub block: String,
    /// Where to align the element horizontally
    pub inline: String,
    /// Offset in pixels for the top position
    pub offset_top: Option<i32>,
    /// Offset in pixels for the left position
    pub offset_left: Option<i32>,
}

impl Default for ScrollToOptions {
    fn default() -> Self {
        Self {
            behavior: ScrollBehavior::Smooth,
            block: "start".to_string(),
            inline: "nearest".to_string(),
            offset_top: None,
            offset_left: None,
        }
    }
}

/// Hook for scrolling to elements or positions on the page
///
/// # Returns
///
/// A tuple containing functions to:
/// 1. Scroll to an element
/// 2. Scroll to a specific position
/// 3. Scroll to the top of the page
/// 4. Scroll to the bottom of the page
///
/// # Example
///
/// ```rust
/// use yewuse::ui::use_scroll_to;
/// use yew::prelude::*;
///
/// #[function_component(ScrollDemo)]
/// fn scroll_demo() -> Html {
///     let section_ref = use_node_ref();
///     let (scroll_to_element, scroll_to_position, scroll_to_top, scroll_to_bottom) = use_scroll_to();
///     
///     let go_to_section = {
///         let section_ref = section_ref.clone();
///         Callback::from(move |_| {
///             scroll_to_element(section_ref.clone(), None);
///         })
///     };
///     
///     let go_to_position = Callback::from(move |_| {
///         scroll_to_position(0, 500, None);
///     });
///     
///     html! {
///         <div>
///             <div style="position: fixed; top: 20px; left: 20px; z-index: 100; background: white; padding: 10px;">
///                 <button onclick={go_to_section}>{"Scroll to Section"}</button>
///                 <button onclick={go_to_position}>{"Scroll to Y=500"}</button>
///                 <button onclick={move |_| scroll_to_top()}>{"Scroll to Top"}</button>
///                 <button onclick={move |_| scroll_to_bottom()}>{"Scroll to Bottom"}</button>
///             </div>
///             
///             <div style="height: 200vh; padding-top: 100px;">
///                 <p>{"Scroll down to see the target section..."}</p>
///                 
///                 <div ref={section_ref} style="margin-top: 1000px; padding: 20px; background: #f0f0f0;">
///                     <h2>{"Target Section"}</h2>
///                     <p>{"You scrolled to this section!"}</p>
///                 </div>
///             </div>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_scroll_to() -> (
    Callback<(NodeRef, Option<ScrollToOptions>)>,   // scroll_to_element
    Callback<(i32, i32, Option<ScrollToOptions>)>,  // scroll_to_position
    Callback<()>,                                  // scroll_to_top
    Callback<()>                                   // scroll_to_bottom
) {
    // Function to scroll to an element
    let scroll_to_element = Callback::from(|(target_ref, options): (NodeRef, Option<ScrollToOptions>)| {
        if let Some(element) = target_ref.cast::<Element>() {
            let options = options.unwrap_or_default();
            
            // Check if we need to apply offset
            let has_offset = options.offset_top.is_some() || options.offset_left.is_some();
            
            if has_offset {
                // If we need offset, we need to calculate the position manually
                if let Ok(rect) = element.get_bounding_client_rect() {
                    let window = window().unwrap();
                    let document = window.document().unwrap();
                    let document_element = document.document_element().unwrap();
                    
                    let current_scroll_x = window.scroll_x().unwrap_or(0.0);
                    let current_scroll_y = window.scroll_y().unwrap_or(0.0);
                    
                    let element_left = rect.left() + current_scroll_x;
                    let element_top = rect.top() + current_scroll_y;
                    
                    // Apply offset if provided
                    let target_x = element_left - options.offset_left.unwrap_or(0) as f64;
                    let target_y = element_top - options.offset_top.unwrap_or(0) as f64;
                    
                    let mut scroll_options = web_sys::ScrollToOptions::new();
                    scroll_options.left(target_x);
                    scroll_options.top(target_y);
                    scroll_options.behavior(options.behavior);
                    
                    window.scroll_with_scroll_to_options(&scroll_options);
                }
            } else {
                // If no offset needed, use scrollIntoView which has more options for alignment
                let mut scroll_options = ScrollIntoViewOptions::new();
                scroll_options.behavior(options.behavior);
                scroll_options.block(&options.block);
                scroll_options.inline(&options.inline);
                
                element.scroll_into_view_with_scroll_into_view_options(&scroll_options);
            }
        }
    });
    
    // Function to scroll to a specific position
    let scroll_to_position = Callback::from(|(left, top, options): (i32, i32, Option<ScrollToOptions>)| {
        let window = window().unwrap();
        let options = options.unwrap_or_default();
        
        let mut scroll_options = web_sys::ScrollToOptions::new();
        scroll_options.left(left as f64);
        scroll_options.top(top as f64);
        scroll_options.behavior(options.behavior);
        
        window.scroll_with_scroll_to_options(&scroll_options);
    });
    
    // Function to scroll to the top of the page
    let scroll_to_top = Callback::from(|_| {
        let window = window().unwrap();
        let mut scroll_options = web_sys::ScrollToOptions::new();
        scroll_options.top(0.0);
        scroll_options.behavior(ScrollBehavior::Smooth);
        
        window.scroll_with_scroll_to_options(&scroll_options);
    });
    
    // Function to scroll to the bottom of the page
    let scroll_to_bottom = Callback::from(|_| {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let document_element = document.document_element().unwrap();
        
        // Get the full height of the document
        let scroll_height = document_element.scroll_height() as f64;
        
        let mut scroll_options = web_sys::ScrollToOptions::new();
        scroll_options.top(scroll_height);
        scroll_options.behavior(ScrollBehavior::Smooth);
        
        window.scroll_with_scroll_to_options(&scroll_options);
    });
    
    (scroll_to_element, scroll_to_position, scroll_to_top, scroll_to_bottom)
}