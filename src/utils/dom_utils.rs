use wasm_bindgen::JsCast;
use web_sys::{window, Document, Element, HtmlElement, Node};

/// DOM utility functions for Yew applications

/// Get the document object
pub fn get_document() -> Option<Document> {
    window().and_then(|win| win.document())
}

/// Query for a single element matching a CSS selector
pub fn query_selector(selector: &str) -> Option<Element> {
    get_document().and_then(|doc| doc.query_selector(selector).ok().flatten())
}

/// Query for all elements matching a CSS selector
pub fn query_selector_all(selector: &str) -> Vec<Element> {
    if let Some(doc) = get_document() {
        if let Ok(node_list) = doc.query_selector_all(selector) {
            let mut elements = Vec::new();
            for i in 0..node_list.length() {
                if let Some(node) = node_list.get(i) {
                    if let Ok(element) = node.dyn_into::<Element>() {
                        elements.push(element);
                    }
                }
            }
            return elements;
        }
    }
    Vec::new()
}

/// Get the active (focused) element
pub fn get_active_element() -> Option<Element> {
    get_document().and_then(|doc| doc.active_element())
}

/// Check if an element has a specific class
pub fn has_class(element: &Element, class_name: &str) -> bool {
    element.class_list().contains(class_name)
}

/// Add a class to an element
pub fn add_class(element: &Element, class_name: &str) {
    let _ = element.class_list().add_1(class_name);
}

/// Remove a class from an element
pub fn remove_class(element: &Element, class_name: &str) {
    let _ = element.class_list().remove_1(class_name);
}

/// Toggle a class on an element
pub fn toggle_class(element: &Element, class_name: &str) {
    let _ = element.class_list().toggle(class_name);
}

/// Get computed style property value
pub fn get_computed_style(element: &Element, property: &str) -> Option<String> {
    window()
        .and_then(|win| win.get_computed_style(element).ok().flatten())
        .and_then(|style| style.get_property_value(property).ok())
}

/// Set element scroll position
pub fn set_scroll_position(element: &Element, x: i32, y: i32) {
    element.set_scroll_left(x);
    element.set_scroll_top(y);
}

/// Scroll element into view
pub fn scroll_into_view(element: &Element, smooth: bool) {
    if smooth {
        let _ = js_sys::Reflect::set(
            &js_sys::Object::new(),
            &"behavior".into(),
            &"smooth".into(),
        );
        element.scroll_into_view_with_bool(true);
    } else {
        element.scroll_into_view();
    }
}

/// Focus an element
pub fn focus_element(element: &Element) {
    if let Ok(html_element) = element.dyn_into::<HtmlElement>() {
        let _ = html_element.focus();
    }
}

/// Blur an element (remove focus)
pub fn blur_element(element: &Element) {
    if let Ok(html_element) = element.dyn_into::<HtmlElement>() {
        let _ = html_element.blur();
    }
}

/// Get the value of an input element
pub fn get_input_value(element: &Element) -> Option<String> {
    element
        .dyn_into::<web_sys::HtmlInputElement>()
        .map(|input| input.value())
        .ok()
        .or_else(|| {
            element
                .dyn_into::<web_sys::HtmlTextAreaElement>()
                .map(|textarea| textarea.value())
                .ok()
        })
        .or_else(|| {
            element
                .dyn_into::<web_sys::HtmlSelectElement>()
                .map(|select| select.value())
                .ok()
        })
}

/// Set the value of an input element
pub fn set_input_value(element: &Element, value: &str) -> bool {
    if let Ok(input) = element.dyn_into::<web_sys::HtmlInputElement>() {
        input.set_value(value);
        return true;
    }
    
    if let Ok(textarea) = element.dyn_into::<web_sys::HtmlTextAreaElement>() {
        textarea.set_value(value);
        return true;
    }
    
    if let Ok(select) = element.dyn_into::<web_sys::HtmlSelectElement>() {
        select.set_value(value);
        return true;
    }
    
    false
}

/// Check if an element is visible (not display: none and not visibility: hidden)
pub fn is_element_visible(element: &Element) -> bool {
    if let Some(style) = get_computed_style(element, "display") {
        if style == "none" {
            return false;
        }
    }
    
    if let Some(style) = get_computed_style(element, "visibility") {
        if style == "hidden" {
            return false;
        }
    }
    
    true
}

/// Get the parent element
pub fn get_parent_element(element: &Element) -> Option<Element> {
    element.parent_element()
}

/// Get all child elements
pub fn get_child_elements(element: &Element) -> Vec<Element> {
    let children = element.children();
    let mut elements = Vec::new();
    
    for i in 0..children.length() {
        if let Some(child) = children.item(i) {
            elements.push(child);
        }
    }
    
    elements
}

/// Append a child element
pub fn append_child(parent: &Element, child: &Node) -> Option<Node> {
    parent.append_child(child).ok()
}

/// Remove a child element
pub fn remove_child(parent: &Element, child: &Node) -> Option<Node> {
    parent.remove_child(child).ok()
}

/// Create a new element
pub fn create_element(tag: &str) -> Option<Element> {
    get_document().and_then(|doc| doc.create_element(tag).ok())
}

/// Create a text node
pub fn create_text_node(text: &str) -> Option<Node> {
    get_document().map(|doc| doc.create_text_node(text))
}

/// Set attribute on an element
pub fn set_attribute(element: &Element, name: &str, value: &str) -> bool {
    element.set_attribute(name, value).is_ok()
}

/// Get attribute from an element
pub fn get_attribute(element: &Element, name: &str) -> Option<String> {
    element.get_attribute(name)
}

/// Remove attribute from an element
pub fn remove_attribute(element: &Element, name: &str) -> bool {
    element.remove_attribute(name).is_ok()
}