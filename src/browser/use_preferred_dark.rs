use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::window;
use yew::prelude::*;

/// Hook for detecting user's preferred color scheme (light/dark)
///
/// # Returns
///
/// A boolean that is true when the user prefers dark mode
///
/// # Example
///
/// ```rust
/// use yewuse::browser::use_preferred_dark;
/// use yew::prelude::*;
///
/// #[function_component(DarkModeAware)]
/// fn dark_mode_aware() -> Html {
///     let prefers_dark = use_preferred_dark();
///     
///     html! {
///         <div class={if *prefers_dark { "dark-theme" } else { "light-theme" }}>
///             <p>{"This component automatically adjusts to your system theme preference."}</p>
///             <p>{"Current theme: "}{if *prefers_dark { "Dark" } else { "Light" }}</p>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_preferred_dark() -> UseStateHandle<bool> {
    let prefers_dark = use_state(|| false);
    
    use_effect_with_deps(
        |_| {
            let window = window().unwrap();
            let media_query = window
                .match_media("(prefers-color-scheme: dark)")
                .unwrap()
                .unwrap();
            
            // Set initial value
            prefers_dark.set(media_query.matches());
            
            // Listen for changes
            let prefers_dark_clone = prefers_dark.clone();
            let listener = Closure::wrap(Box::new(move |e: web_sys::MediaQueryListEvent| {
                prefers_dark_clone.set(e.matches());
            }) as Box<dyn FnMut(_)>);
            
            media_query
                .add_event_listener_with_callback("change", listener.as_ref().unchecked_ref())
                .unwrap();
            
            // Cleanup function
            move || {
                media_query
                    .remove_event_listener_with_callback("change", listener.as_ref().unchecked_ref())
                    .unwrap();
            }
        },
        (),
    );
    
    prefers_dark
}