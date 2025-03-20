use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, HtmlDocument};
use yew::prelude::*;

/// Result of clipboard operations
#[derive(Debug, Clone, PartialEq)]
pub enum ClipboardResult {
    /// Operation succeeded
    Success,
    /// Operation failed with error message
    Error(String),
    /// No operation has been performed yet
    Idle,
}

/// Hook for interacting with the clipboard
///
/// # Returns
///
/// A tuple containing:
/// - The current clipboard operation result
/// - A function to copy text to clipboard
/// - A function to read text from clipboard (if supported by browser)
///
/// # Example
///
/// ```rust
/// use yewuse::browser::use_clipboard;
///
/// #[function_component(ClipboardDemo)]
/// fn clipboard_demo() -> Html {
///     let (result, copy_fn, read_fn) = use_clipboard();
///     let input_ref = use_node_ref();
///     
///     let on_copy = {
///         let input_ref = input_ref.clone();
///         Callback::from(move |_| {
///             if let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>() {
///                 copy_fn(input.value());
///             }
///         })
///     };
///     
///     html! {
///         <div>
///             <input ref={input_ref} type="text" placeholder="Text to copy"/>
///             <button onclick={on_copy}>{"Copy to clipboard"}</button>
///             <button onclick={move |_| read_fn()}>{"Read from clipboard"}</button>
///             <p>{"Status: "}{format!("{:?}", result)}</p>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_clipboard() -> (UseStateHandle<ClipboardResult>, Callback<String>, Callback<()>) {
    let result = use_state(|| ClipboardResult::Idle);
    
    // Copy to clipboard function
    let copy_result = result.clone();
    let copy = Callback::from(move |text: String| {
        let copy_result = copy_result.clone();
        
        if let Some(window) = window() {
            if let Some(navigator) = window.navigator().clipboard() {
                let text_to_copy = text.clone();
                
                copy_result.set(ClipboardResult::Idle);
                
                spawn_local(async move {
                    match navigator.write_text(&text_to_copy).await {
                        Ok(_) => copy_result.set(ClipboardResult::Success),
                        Err(err) => copy_result.set(ClipboardResult::Error(format!("{:?}", err))),
                    }
                });
                
                return;
            }
            
            // Fallback for browsers without clipboard API
            if let Some(document) = window.document() {
                if let Ok(html_document) = document.dyn_into::<HtmlDocument>() {
                    match html_document.exec_command("copy") {
                        Ok(success) => {
                            if success {
                                copy_result.set(ClipboardResult::Success);
                            } else {
                                copy_result.set(ClipboardResult::Error("Command failed".to_string()));
                            }
                        }
                        Err(_) => copy_result.set(ClipboardResult::Error("Command not supported".to_string())),
                    }
                }
            }
        }
    });
    
    // Read from clipboard function
    let read_result = result.clone();
    let read = Callback::from(move |_: ()| {
        let read_result = read_result.clone();
        
        if let Some(window) = window() {
            if let Some(navigator) = window.navigator().clipboard() {
                read_result.set(ClipboardResult::Idle);
                
                spawn_local(async move {
                    match navigator.read_text().await {
                        Ok(text) => {
                            let document = window().unwrap().document().unwrap();
                            if let Some(active_element) = document.active_element() {
                                if let Some(input) = active_element.dyn_into::<web_sys::HtmlInputElement>().ok() {
                                    input.set_value(&text);
                                    read_result.set(ClipboardResult::Success);
                                    return;
                                } else if let Some(textarea) = active_element.dyn_into::<web_sys::HtmlTextAreaElement>().ok() {
                                    textarea.set_value(&text);
                                    read_result.set(ClipboardResult::Success);
                                    return;
                                }
                            }
                            read_result.set(ClipboardResult::Error("No suitable input element focused".to_string()));
                        }
                        Err(err) => read_result.set(ClipboardResult::Error(format!("{:?}", err))),
                    }
                });
            } else {
                read_result.set(ClipboardResult::Error("Clipboard API not supported".to_string()));
            }
        }
    });
    
    (result, copy, read)
}