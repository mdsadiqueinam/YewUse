use wasm_bindgen_test::*;
use yew::prelude::*;
use web_sys::{MouseEvent, Element, HtmlElement};
use wasm_bindgen::{JsCast, closure::Closure};
use yewuse::ui::use_click_outside;
use web_sys::window;
use std::cell::RefCell;
use std::rc::Rc;

wasm_bindgen_test_configure!(run_in_browser);

// Setup function for click outside test
fn setup_click_outside_test() -> (HtmlElement, HtmlElement, Rc<RefCell<i32>>, Rc<RefCell<bool>>) {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    
    // Clean up any previous test elements
    while let Some(child) = body.first_child() {
        body.remove_child(&child).unwrap();
    }
    
    // Create container for our test
    let host = document.create_element("div").unwrap();
    let host_element: HtmlElement = host.dyn_into().unwrap();
    host_element.set_id("test-host");
    body.append_child(&host_element).unwrap();
    
    // Create container element that we'll monitor for "outside" clicks
    let container = document.create_element("div").unwrap();
    let container_element: HtmlElement = container.dyn_into().unwrap();
    container_element.set_id("container");
    container_element.style().set_property("width", "200px").unwrap();
    container_element.style().set_property("height", "200px").unwrap();
    container_element.style().set_property("background-color", "#f0f0f0").unwrap();
    
    // Create inside and outside elements
    let content = r#"
        <p>Click inside container</p>
        <button id="inside-button">Inside Button</button>
    "#;
    container_element.set_inner_html(content);
    
    // Create outside area with button
    let outside_area = document.create_element("div").unwrap();
    let outside_element: HtmlElement = outside_area.dyn_into().unwrap();
    outside_element.set_id("outside-area");
    outside_element.set_inner_html(r#"<button id="outside-button">Outside Button</button>"#);
    
    // Create status display
    let status_display = document.create_element("div").unwrap();
    let status_element: HtmlElement = status_display.dyn_into().unwrap();
    status_element.set_id("status-display");
    status_element.set_inner_html(r#"
        <p id="click-count">Outside clicks: 0</p>
        <button id="toggle-enabled">Disable</button>
        <p id="is-enabled">Enabled: yes</p>
    "#);
    
    // Add everything to the host
    host_element.append_child(&container_element).unwrap();
    host_element.append_child(&outside_element).unwrap();
    host_element.append_child(&status_element).unwrap();
    
    // Setup state for the test
    let click_count = Rc::new(RefCell::new(0));
    let is_enabled = Rc::new(RefCell::new(true));
    
    // Setup button handlers
    let toggle_btn = document.get_element_by_id("toggle-enabled").unwrap();
    let is_enabled_clone = is_enabled.clone();
    let toggle_handler = Closure::wrap(Box::new(move |_: MouseEvent| {
        let current = *is_enabled_clone.borrow();
        *is_enabled_clone.borrow_mut() = !current;
        
        // Update display
        let doc = window().unwrap().document().unwrap();
        if let Some(el) = doc.get_element_by_id("is-enabled") {
            el.set_text_content(Some(&format!("Enabled: {}", if !current { "no" } else { "yes" })));
        }
        if let Some(btn) = doc.get_element_by_id("toggle-enabled") {
            btn.set_text_content(Some(if !current { "Enable" } else { "Disable" }));
        }
    }) as Box<dyn FnMut(_)>);
    toggle_btn.add_event_listener_with_callback("click", toggle_handler.as_ref().unchecked_ref()).unwrap();
    toggle_handler.forget();
    
    // Setup click outside handler
    let click_count_clone = click_count.clone();
    let handle_click_outside = Closure::wrap(Box::new(move |_: MouseEvent| {
        *click_count_clone.borrow_mut() += 1;
        
        // Update display
        let doc = window().unwrap().document().unwrap();
        if let Some(el) = doc.get_element_by_id("click-count") {
            el.set_text_content(Some(&format!("Outside clicks: {}", *click_count_clone.borrow())));
        }
    }) as Box<dyn FnMut(_)>);
    
    // Setup the document listener for mousedown
    let container_el = document.get_element_by_id("container").unwrap();
    let is_enabled_ref = is_enabled.clone();
    let document_ref = document.clone();
    let document_handler = Closure::wrap(Box::new(move |event: MouseEvent| {
        // Only process if enabled
        if !*is_enabled_ref.borrow() {
            return;
        }
        
        // Get the element that was clicked
        if let Some(target) = event.target() {
            let target_element = target.dyn_into::<Element>().ok();
            
            // Check if the click was outside the referenced element
            if let Some(target_element) = target_element {
                if !container_el.contains(Some(&target_element)) && &container_el != &target_element {
                    handle_click_outside.call0(&JsValue::NULL).unwrap();
                }
            }
        }
    }) as Box<dyn FnMut(_)>);
    
    document.add_event_listener_with_callback("mousedown", document_handler.as_ref().unchecked_ref()).unwrap();
    document_handler.forget();
    
    (host_element, container_element, click_count, is_enabled)
}

// Helper to simulate a mouse click
fn simulate_click(element_id: &str) {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let element = document.get_element_by_id(element_id).unwrap();
    
    // Create a MouseEvent
    let event = document
        .create_event("MouseEvent")
        .unwrap()
        .dyn_into::<MouseEvent>()
        .unwrap();
    
    // Initialize the event
    event
        .init_mouse_event_with_can_bubble_arg_and_cancelable_arg(
            "mousedown",
            true,  // bubbles
            true,  // cancelable
            Some(&window),
            0,     // detail
            0,     // screen_x
            0,     // screen_y
            0,     // client_x
            0,     // client_y
            false, // ctrl_key
            false, // alt_key
            false, // shift_key
            false, // meta_key
            0,     // button
            None,  // related_target
        );
    
    // Dispatch the event
    element.dispatch_event(&event).unwrap();
}

// Helper to get current click count
fn get_click_count() -> i32 {
    let doc = window().unwrap().document().unwrap();
    let count_el = doc.get_element_by_id("click-count").unwrap();
    let text = count_el.text_content().unwrap();
    text.replace("Outside clicks: ", "").parse::<i32>().unwrap()
}

#[wasm_bindgen_test]
fn test_click_outside_detection() {
    // Arrange
    let (host, container, click_count, _) = setup_click_outside_test();
    
    // Act - simulate click outside
    simulate_click("outside-button");
    
    // Small delay to allow for event processing
    let cb = Closure::once(Box::new(move || {}) as Box<dyn FnMut()>);
    window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(
        cb.as_ref().unchecked_ref(),
        10,
    ).unwrap();
    cb.forget();
    
    // Assert - click count should have incremented
    assert_eq!(get_click_count(), 1);
    
    // Cleanup
    window().unwrap().document().unwrap().body().unwrap().remove_child(&host).unwrap();
}

#[wasm_bindgen_test]
fn test_click_inside_not_detected() {
    // Arrange
    let (host, container, click_count, _) = setup_click_outside_test();
    
    // Initial state
    assert_eq!(get_click_count(), 0);
    
    // Act - simulate click inside
    simulate_click("inside-button");
    
    // Small delay to allow for event processing
    let cb = Closure::once(Box::new(move || {}) as Box<dyn FnMut()>);
    window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(
        cb.as_ref().unchecked_ref(),
        10,
    ).unwrap();
    cb.forget();
    
    // Assert - click count should not have changed
    assert_eq!(get_click_count(), 0);
    
    // Cleanup
    window().unwrap().document().unwrap().body().unwrap().remove_child(&host).unwrap();
}

#[wasm_bindgen_test]
fn test_click_outside_disabled() {
    // Arrange
    let (host, container, click_count, is_enabled) = setup_click_outside_test();
    
    // Act - disable the detection
    simulate_click("toggle-enabled");
    
    // Small delay
    let cb1 = Closure::once(Box::new(move || {}) as Box<dyn FnMut()>);
    window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(
        cb1.as_ref().unchecked_ref(),
        10,
    ).unwrap();
    cb1.forget();
    
    // Verify disabled state
    assert_eq!(*is_enabled.borrow(), false);
    
    // Act - click outside while disabled
    simulate_click("outside-button");
    
    // Small delay
    let cb2 = Closure::once(Box::new(move || {}) as Box<dyn FnMut()>);
    window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(
        cb2.as_ref().unchecked_ref(),
        10,
    ).unwrap();
    cb2.forget();
    
    // Assert - click count should not have incremented
    assert_eq!(get_click_count(), 0);
    
    // Cleanup
    window().unwrap().document().unwrap().body().unwrap().remove_child(&host).unwrap();
}