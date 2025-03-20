use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{window, Element, Event};
use yew::prelude::*;

/// Scroll position information
#[derive(Debug, Clone, PartialEq)]
pub struct ScrollInfo {
    /// Scroll X position in pixels
    pub x: i32,
    /// Scroll Y position in pixels
    pub y: i32,
    /// Width of the scrollable area
    pub width: i32,
    /// Height of the scrollable area
    pub height: i32,
    /// Maximum scroll X position
    pub max_x: i32,
    /// Maximum scroll Y position
    pub max_y: i32,
    /// Percentage scrolled horizontally (0-100)
    pub percentage_x: f64,
    /// Percentage scrolled vertically (0-100)
    pub percentage_y: f64,
    /// Direction of scroll, true if scrolling down
    pub is_scrolling_down: Option<bool>,
    /// Direction of scroll, true if scrolling right
    pub is_scrolling_right: Option<bool>,
}

/// Hook for tracking scroll position of an element or the window
///
/// # Arguments
///
/// * `target_ref` - Optional reference to the element to track scrolling of (None for window)
///
/// # Returns
///
/// The current scroll information
///
/// # Example
///
/// ```rust
/// use yewuse::ui::use_scroll;
/// use yew::prelude::*;
///
/// #[function_component(ScrollTracker)]
/// fn scroll_tracker() -> Html {
///     let scroll_info = use_scroll(None); // Track window scrolling
///     
///     html! {
///         <div class="scroll-tracker">
///             <p>{"Scroll Y: "}{scroll_info.y}{"px"}</p>
///             <p>{"Scroll percentage: "}{format!("{:.1}%", scroll_info.percentage_y)}</p>
///             <p>{"Scrolling direction: "}{
///                 if let Some(is_down) = scroll_info.is_scrolling_down {
///                     if is_down { "Down" } else { "Up" }
///                 } else {
///                     "Not scrolling"
///                 }
///             }</p>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_scroll(target_ref: Option<NodeRef>) -> ScrollInfo {
    let scroll_info = use_state(|| ScrollInfo {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        max_x: 0,
        max_y: 0,
        percentage_x: 0.0,
        percentage_y: 0.0,
        is_scrolling_down: None,
        is_scrolling_right: None,
    });
    
    let prev_y = use_ref(|| 0);
    let prev_x = use_ref(|| 0);
    
    use_effect_with_deps(
        move |target_ref| {
            let window = window().unwrap();
            let scroll_info_clone = scroll_info.clone();
            let prev_y_clone = prev_y.clone();
            let prev_x_clone = prev_x.clone();
            
            // Function to get scroll information from an element or window
            let update_scroll_info = {
                let scroll_info_clone = scroll_info_clone.clone();
                let prev_y_clone = prev_y_clone.clone();
                let prev_x_clone = prev_x_clone.clone();
                
                move || {
                    let (x, y, width, height, max_x, max_y) = if let Some(target_ref) = target_ref {
                        if let Some(element) = target_ref.cast::<Element>() {
                            let scroll_left = element.scroll_left();
                            let scroll_top = element.scroll_top();
                            let scroll_width = element.scroll_width();
                            let scroll_height = element.scroll_height();
                            let client_width = element.client_width();
                            let client_height = element.client_height();
                            
                            (
                                scroll_left,
                                scroll_top,
                                scroll_width,
                                scroll_height,
                                scroll_width - client_width,
                                scroll_height - client_height,
                            )
                        } else {
                            (0, 0, 0, 0, 0, 0)
                        }
                    } else {
                        // Window scrolling
                        (
                            window.scroll_x().unwrap_or(0.0) as i32,
                            window.scroll_y().unwrap_or(0.0) as i32,
                            window.inner_width().unwrap().as_f64().unwrap() as i32,
                            window.inner_height().unwrap().as_f64().unwrap() as i32,
                            window.document().unwrap().body().unwrap().scroll_width() - window.inner_width().unwrap().as_f64().unwrap() as i32,
                            window.document().unwrap().body().unwrap().scroll_height() - window.inner_height().unwrap().as_f64().unwrap() as i32,
                        )
                    };
                    
                    // Calculate scroll direction
                    let prev_y_value = *prev_y_clone.borrow();
                    let prev_x_value = *prev_x_clone.borrow();
                    
                    let is_scrolling_down = if y != prev_y_value {
                        Some(y > prev_y_value)
                    } else {
                        None
                    };
                    
                    let is_scrolling_right = if x != prev_x_value {
                        Some(x > prev_x_value)
                    } else {
                        None
                    };
                    
                    // Update previous positions
                    *prev_y_clone.borrow_mut() = y;
                    *prev_x_clone.borrow_mut() = x;
                    
                    // Calculate percentages
                    let percentage_x = if max_x <= 0 {
                        0.0
                    } else {
                        (x as f64 / max_x as f64) * 100.0
                    };
                    
                    let percentage_y = if max_y <= 0 {
                        0.0
                    } else {
                        (y as f64 / max_y as f64) * 100.0
                    };
                    
                    // Update state
                    scroll_info_clone.set(ScrollInfo {
                        x,
                        y,
                        width,
                        height,
                        max_x,
                        max_y,
                        percentage_x,
                        percentage_y,
                        is_scrolling_down,
                        is_scrolling_right,
                    });
                }
            };
            
            // Initialize scroll info
            update_scroll_info();
            
            // Create scroll event listener
            let callback = Closure::wrap(Box::new(move |_e: Event| {
                update_scroll_info();
            }) as Box<dyn FnMut(_)>);
            
            // Attach listener to the appropriate target
            if let Some(target_ref) = target_ref {
                if let Some(element) = target_ref.cast::<Element>() {
                    element
                        .add_event_listener_with_callback("scroll", callback.as_ref().unchecked_ref())
                        .unwrap();
                }
            } else {
                window
                    .add_event_listener_with_callback("scroll", callback.as_ref().unchecked_ref())
                    .unwrap();
            }
            
            // Also listen for window resize to update max scroll values
            let resize_callback = Closure::wrap(Box::new(move |_e: Event| {
                update_scroll_info();
            }) as Box<dyn FnMut(_)>);
            
            window
                .add_event_listener_with_callback("resize", resize_callback.as_ref().unchecked_ref())
                .unwrap();
            
            // Cleanup function
            move || {
                if let Some(target_ref) = target_ref {
                    if let Some(element) = target_ref.cast::<Element>() {
                        element
                            .remove_event_listener_with_callback("scroll", callback.as_ref().unchecked_ref())
                            .unwrap();
                    }
                } else {
                    window
                        .remove_event_listener_with_callback("scroll", callback.as_ref().unchecked_ref())
                        .unwrap();
                }
                
                window
                    .remove_event_listener_with_callback("resize", resize_callback.as_ref().unchecked_ref())
                    .unwrap();
            }
        },
        target_ref,
    );
    
    (*scroll_info).clone()
}