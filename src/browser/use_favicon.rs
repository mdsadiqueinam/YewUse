use wasm_bindgen::JsCast;
use web_sys::{window, HtmlLinkElement};
use yew::hook;

#[derive(Clone)]
pub struct UseFaviconOptions {
    pub force_refresh: bool,
    pub base_url: Option<String>,
    pub rel: String,
}

impl Default for UseFaviconOptions {
    fn default() -> Self {
        Self {
            force_refresh: false,
            base_url: None,
            rel: "shortcut icon".to_string(),
        }
    }
}

#[hook]
pub fn use_favicon(initial_icon: Option<String>, options: Option<UseFaviconOptions>) -> impl Fn(String) {
    let options = options.unwrap_or_default();
    let current_icon = yew::use_state(|| initial_icon.unwrap_or_default());
    
    // Create or find existing favicon link element
    let link_element = {
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                let existing = document
                    .query_selector(&format!("link[rel*='icon']"))
                    .ok()
                    .flatten();

                match existing {
                    Some(element) => element.dyn_into::<HtmlLinkElement>().ok(),
                    None => {
                        document
                            .create_element("link")
                            .ok()
                            .and_then(|el| el.dyn_into::<HtmlLinkElement>().ok())
                            .map(|link| {
                                link.set_rel(&options.rel);
                                document.head().unwrap().append_child(&link).unwrap();
                                link
                            })
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    };

    move |icon: String| {
        if let Some(link) = &link_element {
            let mut href = icon.clone();
            
            // Add base URL if provided
            if let Some(base) = &options.base_url {
                href = format!("{}{}", base, href);
            }

            // Add cache-busting query parameter if force refresh is enabled
            if options.force_refresh {
                let timestamp = js_sys::Date::now();
                href = format!("{}?v={}", href, timestamp);
            }

            link.set_href(&href);
            current_icon.set(icon);
        }
    }
}