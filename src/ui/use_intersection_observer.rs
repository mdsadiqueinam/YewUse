use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{window, Element, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};
use yew::prelude::*;

/// Configuration options for the Intersection Observer
#[derive(Clone)]
pub struct UseIntersectionOptions {
    /// The element that is used as the viewport for checking visibility
    pub root: Option<NodeRef>,
    /// Margin around the root element
    pub root_margin: Option<String>,
    /// Threshold(s) at which to trigger callbacks
    pub threshold: Vec<f64>,
}

impl Default for UseIntersectionOptions {
    fn default() -> Self {
        Self {
            root: None,
            root_margin: None,
            threshold: vec![0.0],
        }
    }
}

/// Information about the intersection state
#[derive(Clone, Debug)]
pub struct IntersectionInfo {
    /// Whether the target element is currently intersecting with the root
    pub is_intersecting: bool,
    /// Ratio of the target element that is visible (0-1)
    pub intersection_ratio: f64,
    /// Bounding client rect of the target element
    pub bounding_client_rect: web_sys::DomRectReadOnly,
    /// Intersection rect
    pub intersection_rect: web_sys::DomRectReadOnly,
    /// Is the target fully visible in the root?
    pub is_fully_visible: bool,
    /// Time when the intersection occurred
    pub time: f64,
}

impl Default for IntersectionInfo {
    fn default() -> Self {
        Self {
            is_intersecting: false,
            intersection_ratio: 0.0,
            bounding_client_rect: web_sys::DomRectReadOnly::new().unwrap(),
            intersection_rect: web_sys::DomRectReadOnly::new().unwrap(),
            is_fully_visible: false,
            time: 0.0,
        }
    }
}

/// Hook for detecting when an element enters or exits the viewport
///
/// # Arguments
///
/// * `target` - Element to observe
/// * `options` - Optional configuration for the Intersection Observer
///
/// # Returns
///
/// Current intersection information
///
/// # Example
///
/// ```rust
/// use yewuse::ui::use_intersection_observer;
/// use yew::prelude::*;
///
/// #[function_component(LazyImage)]
/// fn lazy_image() -> Html {
///     let image_ref = use_node_ref();
///     let intersection = use_intersection_observer(image_ref.clone(), None);
///     
///     // Only load the image when it comes into view
///     let image_src = if intersection.is_intersecting {
///         "https://example.com/actual-image.jpg"
///     } else {
///         "data:image/gif;base64,R0lGODlhAQABAAAAACH5BAEKAAEALAAAAAABAAEAAAICTAEAOw==" // Tiny placeholder
///     };
///     
///     html! {
///         <div class="lazy-image-container">
///             <img 
///                 ref={image_ref}
///                 src={image_src}
///                 alt="Lazy loaded image"
///                 loading="lazy"
///             />
///             <p>{"Visibility: "}{if intersection.is_intersecting { "Visible" } else { "Hidden" }}</p>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_intersection_observer(
    target: NodeRef,
    options: Option<UseIntersectionOptions>,
) -> IntersectionInfo {
    let intersection_info = use_state(IntersectionInfo::default);
    let observer_ref = use_mut_ref(|| None::<IntersectionObserver>);
    
    let options = options.unwrap_or_default();
    
    use_effect_with_deps(
        move |(target, options)| {
            let intersection_info = intersection_info.clone();
            
            // Create intersection observer
            let callback = {
                let intersection_info = intersection_info.clone();
                
                Closure::wrap(Box::new(move |entries: js_sys::Array, _observer: IntersectionObserver| {
                    // We're only observing one element, so we can use the first entry
                    if let Some(entry) = entries.get(0).dyn_into::<IntersectionObserverEntry>().ok() {
                        intersection_info.set(IntersectionInfo {
                            is_intersecting: entry.is_intersecting(),
                            intersection_ratio: entry.intersection_ratio(),
                            bounding_client_rect: entry.bounding_client_rect(),
                            intersection_rect: entry.intersection_rect(),
                            is_fully_visible: entry.intersection_ratio() >= 0.99,
                            time: entry.time(),
                        });
                    }
                }) as Box<dyn FnMut(js_sys::Array, IntersectionObserver)>)
            };
            
            // Configure the observer
            let mut observer_init = IntersectionObserverInit::new();
            
            // Set root element if provided
            if let Some(root_ref) = &options.root {
                if let Some(root_element) = root_ref.cast::<Element>() {
                    observer_init.root(Some(&root_element));
                }
            }
            
            // Set root margin if provided
            if let Some(margin) = &options.root_margin {
                observer_init.root_margin(margin);
            }
            
            // Set threshold
            let thresholds = js_sys::Array::new();
            for threshold in &options.threshold {
                thresholds.push(&(*threshold).into());
            }
            observer_init.threshold(&thresholds);
            
            // Create and store the observer
            if let Ok(observer) = IntersectionObserver::new_with_options(
                callback.as_ref().unchecked_ref(),
                &observer_init,
            ) {
                // Start observing the target element
                if let Some(element) = target.cast::<Element>() {
                    observer.observe(&element);
                    *observer_ref.borrow_mut() = Some(observer);
                }
                
                // Keep the callback alive
                callback.forget();
            }
            
            // Cleanup when the component unmounts
            move || {
                if let Some(observer) = observer_ref.borrow_mut().take() {
                    observer.disconnect();
                }
            }
        },
        (target, options),
    );
    
    (*intersection_info).clone()
}