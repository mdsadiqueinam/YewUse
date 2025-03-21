use wasm_bindgen::JsCast;
use web_sys::{window, Element};
use yew::hook;

#[derive(Clone)]
pub struct ScrollToOptions {
    pub behavior: ScrollBehavior,
    pub left: Option<f64>,
    pub top: Option<f64>,
}

#[derive(Clone)]
pub enum ScrollBehavior {
    Auto,
    Smooth,
    Instant,
}

impl ToString for ScrollBehavior {
    fn to_string(&self) -> String {
        match self {
            ScrollBehavior::Auto => "auto",
            ScrollBehavior::Smooth => "smooth",
            ScrollBehavior::Instant => "instant",
        }
        .to_string()
    }
}

impl Default for ScrollToOptions {
    fn default() -> Self {
        Self {
            behavior: ScrollBehavior::Auto,
            left: None,
            top: None,
        }
    }
}

#[hook]
pub fn use_scroll_to() -> impl Fn(ScrollToOptions) {
    move |options: ScrollToOptions| {
        if let Some(window) = window() {
            let mut opts = web_sys::ScrollToOptions::new();
            opts.behavior(&options.behavior.to_string());
            
            if let Some(left) = options.left {
                opts.left(left);
            }
            if let Some(top) = options.top {
                opts.top(top);
            }
            
            window.scroll_with_scroll_to_options(&opts);
        }
    }
}

#[hook]
pub fn use_element_scroll_to(element: Element) -> impl Fn(ScrollToOptions) {
    move |options: ScrollToOptions| {
        let mut opts = web_sys::ScrollToOptions::new();
        opts.behavior(&options.behavior.to_string());
        
        if let Some(left) = options.left {
            opts.left(left);
        }
        if let Some(top) = options.top {
            opts.top(top);
        }
        
        element.scroll_with_scroll_to_options(&opts);
    }
}

#[hook]
pub fn use_scroll_into_view(element: Element) -> impl Fn(ScrollBehavior) {
    move |behavior: ScrollBehavior| {
        let mut opts = web_sys::ScrollIntoViewOptions::new();
        opts.behavior(&behavior.to_string());
        element.scroll_into_view_with_scroll_into_view_options(&opts);
    }
}