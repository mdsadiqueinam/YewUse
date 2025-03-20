use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{window, Element, ResizeObserver, ResizeObserverEntry};
use yew::prelude::*;

/// Information about an element's size and position
#[derive(Clone, Debug, PartialEq)]
pub struct ElementSize {
    /// Width of the element in pixels
    pub width: f64,
    /// Height of the element in pixels
    pub height: f64,
    /// Left position relative to the viewport
    pub left: f64,
    /// Top position relative to the viewport
    pub top: f64,
    /// Element's x position relative to the document
    pub x: f64,
    /// Element's y position relative to the document
    pub y: f64,
    /// Element's right edge position
    pub right: f64,
    /// Element's bottom edge position
    pub bottom: f64,
}

impl Default for ElementSize {
    fn default() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
            left: 0.0,
            top: 0.0,
            x: 0.0,
            y: 0.0,
            right: 0.0,
            bottom: 0.0,
        }
    }
}

/// Options for the use_element_size hook
#[derive(Clone, PartialEq)]
pub struct UseElementSizeOptions {
    /// Whether to use the ResizeObserver API
    pub use_resize_observer: bool,
    /// Whether to track position as well as size
    pub track_position: bool,
    /// Whether to update on window resize
    pub watch_window_resize: bool,
}

impl Default for UseElementSizeOptions {
    fn default() -> Self {
        Self {
            use_resize_observer: true,
            track_position: true,
            watch_window_resize: true,
        }
    }
}

/// Hook to track the size and position of a DOM element
///
/// # Arguments
///
/// * `target` - Reference to the element to track
/// * `options` - Optional configuration for the size tracking
///
/// # Returns
///
/// The current size and position of the element
///
/// # Example
///
/// ```rust
/// use yewuse::ui::use_element_size;
/// use yew::prelude::*;
///
/// #[function_component(ResizableBox)]
/// fn resizable_box() -> Html {
///     let box_ref = use_node_ref();
///     let size = use_element_size(box_ref.clone(), None);
///     
///     html! {
///         <div>
///             <div ref={box_ref} class="resizable-box" style="resize: both; overflow: auto; width: 200px; height: 100px; border: 1px solid black;">
///                 <p>{"Resize me!"}</p>
///             </div>
///             <p>{"Current size: "}{size.width.round()}{" x "}{size.height.round()}{" pixels"}</p>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_element_size(
    target: NodeRef,
    options: Option<UseElementSizeOptions>,
) -> ElementSize {
    let size = use_state(ElementSize::default);
    let options = options.unwrap_or_default();
    let observer_ref = use_mut_ref(|| None::<ResizeObserver>);
    
    // Helper function to update the element size
    let update_element_size = {
        let size = size.clone();
        let target = target.clone();
        let track_position = options.track_position;
        
        move || {
            if let Some(element) = target.cast::<Element>() {
                if let Ok(rect) = element.get_bounding_client_rect() {
                    let mut new_size = ElementSize {
                        width: rect.width(),
                        height: rect.height(),
                        left: rect.left(),
                        top: rect.top(),
                        right: rect.right(),
                        bottom: rect.bottom(),
                        x: 0.0,
                        y: 0.0,
                    };
                    
                    // Get scroll position for absolute coordinates if needed
                    if track_position {
                        let window = window().unwrap();
                        let scroll_x = window.scroll_x().unwrap_or(0.0);
                        let scroll_y = window.scroll_y().unwrap_or(0.0);
                        
                        new_size.x = rect.left() + scroll_x;
                        new_size.y = rect.top() + scroll_y;
                    }
                    
                    size.set(new_size);
                }
            }
        }
    };
    
    // Set up the resize observer and/or window resize listener
    use_effect_with_deps(
        move |(target, options)| {
            let update = update_element_size.clone();
            let mut listeners = Vec::new();
            
            // Try to get the element once it's mounted
            let element = target.cast::<Element>();
            
            // Set up ResizeObserver if requested and available
            if options.use_resize_observer {
                if let Some(element) = &element {
                    let callback = {
                        let update = update.clone();
                        
                        Closure::wrap(Box::new(move |entries: js_sys::Array, _observer: ResizeObserver| {
                            // We're only observing one element, so we can use the first entry
                            if let Some(_entry) = entries.get(0).dyn_into::<ResizeObserverEntry>().ok() {
                                update();
                            }
                        }) as Box<dyn FnMut(js_sys::Array, ResizeObserver)>)
                    };
                    
                    if let Ok(observer) = ResizeObserver::new(callback.as_ref().unchecked_ref()) {
                        observer.observe(element);
                        *observer_ref.borrow_mut() = Some(observer);
                        
                        // Keep the callback alive
                        callback.forget();
                    }
                }
            }
            
            // Set up window resize listener if requested
            if options.watch_window_resize {
                let window = window().unwrap();
                let resize_callback = {
                    let update = update.clone();
                    
                    Closure::wrap(Box::new(move |_: web_sys::Event| {
                        update();
                    }) as Box<dyn FnMut(_)>)
                };
                
                window
                    .add_event_listener_with_callback("resize", resize_callback.as_ref().unchecked_ref())
                    .unwrap();
                
                listeners.push(resize_callback);
            }
            
            // Initial update
            update();
            
            // Cleanup function
            move || {
                // Clean up resize observer
                if let Some(observer) = observer_ref.borrow_mut().take() {
                    observer.disconnect();
                }
                
                // Clean up window resize listener
                if options.watch_window_resize {
                    let window = window().unwrap();
                    
                    for listener in listeners {
                        window
                            .remove_event_listener_with_callback("resize", listener.as_ref().unchecked_ref())
                            .unwrap_or_default();
                    }
                }
            }
        },
        (target, options),
    );
    
    (*size).clone()
}