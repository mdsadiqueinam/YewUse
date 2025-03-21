use wasm_bindgen::JsCast;
use web_sys::Clipboard;
use yew::hook;

#[derive(Clone)]
pub struct UseClipboardReturn {
    pub is_supported: bool,
    pub text: String,
    pub copied: bool,
    pub error: Option<String>,
}

impl Default for UseClipboardReturn {
    fn default() -> Self {
        Self {
            is_supported: false,
            text: String::new(),
            copied: false,
            error: None,
        }
    }
}

#[hook]
pub fn use_clipboard(initial_value: Option<String>) -> (UseClipboardReturn, impl Fn(String) -> ()) {
    let state = yew::use_state(|| UseClipboardReturn {
        is_supported: web_sys::window()
            .and_then(|w| w.navigator().clipboard())
            .is_some(),
        text: initial_value.unwrap_or_default(),
        copied: false,
        error: None,
    });

    let copy = {
        let state = state.clone();
        move |text: String| {
            let state = state.clone();
            if let Some(clipboard) = web_sys::window()
                .and_then(|w| w.navigator().clipboard())
            {
                wasm_bindgen_futures::spawn_local(async move {
                    match clipboard.write_text(&text).await {
                        Ok(_) => {
                            state.set(UseClipboardReturn {
                                is_supported: true,
                                text: text.clone(),
                                copied: true,
                                error: None,
                            });
                        }
                        Err(err) => {
                            state.set(UseClipboardReturn {
                                is_supported: true,
                                text: text.clone(),
                                copied: false,
                                error: Some(format!("Failed to copy: {:?}", err)),
                            });
                        }
                    }
                });
            } else {
                state.set(UseClipboardReturn {
                    is_supported: false,
                    text,
                    copied: false,
                    error: Some("Clipboard API not supported".to_string()),
                });
            }
        }
    };

    ((*state).clone(), copy)
}

#[hook]
pub fn use_clipboard_read() -> impl Fn() -> () {
    let text = yew::use_state(String::new);
    let error = yew::use_state(|| None::<String>);

    move || {
        let text = text.clone();
        let error = error.clone();
        if let Some(clipboard) = web_sys::window()
            .and_then(|w| w.navigator().clipboard())
        {
            wasm_bindgen_futures::spawn_local(async move {
                match clipboard.read_text().await {
                    Ok(content) => {
                        text.set(content);
                        error.set(None);
                    }
                    Err(err) => {
                        error.set(Some(format!("Failed to read clipboard: {:?}", err)));
                    }
                }
            });
        } else {
            error.set(Some("Clipboard API not supported".to_string()));
        }
    }
}