use wasm_bindgen::JsCast;
use web_sys::window;
use yew::prelude::*;

/// Hook for setting and tracking document title
///
/// # Arguments
///
/// * `title` - The title to set on the document
///
/// # Example
///
/// ```rust
/// use yewuse::browser::use_document_title;
/// 
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     use_document_title("My Page Title".to_string());
///     html! { <div>{"Page with custom title"}</div> }
/// }
/// ```
#[hook]
pub fn use_document_title(title: String) {
    use_effect_with_deps(
        move |title| {
            if let Some(window) = window() {
                if let Some(document) = window.document() {
                    document.set_title(title);
                }
            }
            || {}
        },
        title,
    );
}