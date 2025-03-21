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
    use_effect_with(title, move |title| {
        if let Some(document) = window().and_then(|w| w.document()) {
            document.set_title(title);
        }
        || {}
    });
}
