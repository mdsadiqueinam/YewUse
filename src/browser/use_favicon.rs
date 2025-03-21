use wasm_bindgen::JsCast;
use web_sys::window;
use yew::prelude::*;

/// Hook for dynamically changing the website's favicon
///
/// # Arguments
///
/// * `icon_href` - The URL/path to the favicon image
///
/// # Example
///
/// ```rust
/// use yewuse::browser::use_favicon;
/// use yew::prelude::*;
///
/// #[function_component(DynamicFavicon)]
/// fn dynamic_favicon() -> Html {
///     let (icon, set_icon) = use_state(|| "/favicon.ico".to_string());
///     
///     // Change favicon dynamically
///     use_favicon((*icon).clone());
///     
///     let set_normal = {
///         let set_icon = set_icon.clone();
///         Callback::from(move |_| set_icon("/favicon.ico".to_string()))
///     };
///     
///     let set_alert = {
///         let set_icon = set_icon.clone();
///         Callback::from(move |_| set_icon("/alert-favicon.ico".to_string()))
///     };
///     
///     html! {
///         <div>
///             <p>{"Current favicon: "}{(*icon).clone()}</p>
///             <button onclick={set_normal}>{"Normal Favicon"}</button>
///             <button onclick={set_alert}>{"Alert Favicon"}</button>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_favicon(icon_href: String) {
    use_effect_with_deps(
        move |icon_href| {
            if let Some(window) = window() {
                if let Some(document) = window.document() {
                    // Try to find existing favicon link
                    let head = document.head().expect("Document should have head");
                    let existing_icon = document.query_selector("link[rel*='icon']").ok().flatten();
                    
                    if let Some(existing) = existing_icon {
                        // Update existing favicon link
                        existing
                            .dyn_ref::<web_sys::HtmlLinkElement>()
                            .expect("Should be a link element")
                            .set_href(icon_href);
                    } else {
                        // Create new favicon link if none exists
                        if let Ok(link) = document.create_element("link") {
                            let link = link
                                .dyn_into::<web_sys::HtmlLinkElement>()
                                .expect("Should be a link element");
                            
                            link.set_rel("icon");
                            link.set_href(icon_href);
                            
                            head.append_child(&link).expect("Failed to append favicon link");
                        }
                    }
                }
            }
            || {}
        },
        icon_href,
    );
}