use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::window;
use yew::prelude::*;

/// Hook for responding to media query changes
///
/// # Arguments
///
/// * `query` - CSS media query string (e.g., "(max-width: 768px)")
///
/// # Returns
///
/// A boolean indicating whether the media query matches
///
/// # Example
///
/// ```rust
/// use yewuse::sensors::use_media_query;
///
/// #[function_component(ResponsiveComponent)]
/// fn responsive_component() -> Html {
///     let is_mobile = use_media_query("(max-width: 768px)");
///     
///     html! {
///         <div>
///             {if is_mobile {
///                 html! { <p>{"Mobile view"}</p> }
///             } else {
///                 html! { <p>{"Desktop view"}</p> }
///             }}
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_media_query(query: &str) -> bool {
    let matches = use_state(|| false);
    let query = query.to_owned();

    use_effect_with_deps(
        move |query| {
            let window = window().unwrap();
            let media_query_list = window.match_media(query).unwrap().unwrap();
            
            // Set initial value
            matches.set(media_query_list.matches());
            
            // Listen for changes
            let matches_clone = matches.clone();
            let listener = Closure::wrap(Box::new(move |e: web_sys::MediaQueryListEvent| {
                matches_clone.set(e.matches());
            }) as Box<dyn FnMut(_)>);
            
            media_query_list
                .add_event_listener_with_callback("change", listener.as_ref().unchecked_ref())
                .unwrap();
            
            // Cleanup function
            move || {
                media_query_list
                    .remove_event_listener_with_callback("change", listener.as_ref().unchecked_ref())
                    .unwrap();
            }
        },
        query,
    );

    *matches
}