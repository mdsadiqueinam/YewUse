use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{window, Element, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};
use yew::prelude::*;
use yew::hook;

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
pub fn use_intersection_observer<F>(
    target: Element,
    callback: F,
    options: Option<UseIntersectionObserverOptions>,
) where
    F: Fn(&[IntersectionObserverEntry]) + 'static,
{
    let options = options.unwrap_or_default();

    yew::use_effect_with_deps(
        move |(target, options)| {
            let mut observer_init = IntersectionObserverInit::new();
            
            if let Some(root) = &options.root {
                observer_init.root(Some(root));
            }
            
            if let Some(margin) = &options.root_margin {
                observer_init.root_margin(margin);
            }
            
            if let Some(threshold) = &options.threshold {
                observer_init.threshold(&threshold.into());
            }

            let callback = Closure::wrap(Box::new(move |entries: js_sys::Array, _observer: IntersectionObserver| {
                let entries: Vec<IntersectionObserverEntry> = entries
                    .iter()
                    .filter_map(|entry| entry.dyn_into::<IntersectionObserverEntry>().ok())
                    .collect();
                callback(&entries);
            }) as Box<dyn FnMut(js_sys::Array, IntersectionObserver)>);

            let observer = IntersectionObserver::new_with_options(
                callback.as_ref().unchecked_ref(),
                &observer_init,
            )
            .unwrap();

            observer.observe(target);

            move || {
                observer.disconnect();
            }
        },
        (target, options),
    );
}

#[hook]
pub fn use_element_visibility(element: Element) -> bool {
    let is_visible = yew::use_state(|| false);

    {
        let is_visible = is_visible.clone();
        use_intersection_observer(
            element,
            move |entries| {
                if let Some(entry) = entries.first() {
                    is_visible.set(entry.is_intersecting());
                }
            },
            None,
        );
    }

    *is_visible
}