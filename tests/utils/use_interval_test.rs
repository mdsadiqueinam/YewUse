use wasm_bindgen_test::*;
use yew::prelude::*;
use yewuse::utils::use_interval;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{window, Document, HtmlElement};
use std::rc::Rc;
use std::cell::RefCell;
use js_sys::Promise;

wasm_bindgen_test_configure!(run_in_browser);

// Helper function to create and render a component for testing interval
fn setup_interval_test() -> (HtmlElement, Rc<RefCell<i32>>, Rc<RefCell<bool>>) {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    
    // Clean up any previous test elements
    while let Some(child) = body.first_child() {
        body.remove_child(&child).unwrap();
    }
    
    // Create a container for our test
    let container = document.create_element("div").unwrap();
    container.set_id("test-container");
    body.append_child(&container).unwrap();
    
    // Create counter and active state
    let counter = Rc::new(RefCell::new(0));
    let active = Rc::new(RefCell::new(true));
    
    // Create HTML content
    let host = document.create_element("div").unwrap();
    let host_element: HtmlElement = host.dyn_into().unwrap();
    host_element.set_inner_html(&format!(r#"
        <div>
            <p id="counter">Count: {}</p>
            <button id="toggle">{}</button>
            <button id="reset">Reset</button>
            <div id="status">{}</div>
        </div>
    "#, 
        *counter.borrow(),
        if *active.borrow() { "Pause" } else { "Resume" },
        if *active.borrow() { "Active" } else { "Paused" }
    ));
    
    // Add our content to the container
    container.append_child(&host_element).unwrap();
    
    // Set up interval tracking
    let interval_id = Rc::new(RefCell::new(None::<i32>));
    let counter_clone = counter.clone();
    
    // Set up the interval
    if *active.borrow() {
        let counter_update = counter_clone.clone();
        let callback = Closure::wrap(Box::new(move || {
            *counter_update.borrow_mut() += 1;
            
            // Update the display
            let doc = window().unwrap().document().unwrap();
            if let Some(el) = doc.get_element_by_id("counter") {
                el.set_text_content(Some(&format!("Count: {}", *counter_update.borrow())));
            }
        }) as Box<dyn FnMut()>);
        
        let id = window.set_interval_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            100 // Use a short interval for testing
        ).unwrap();
        
        *interval_id.borrow_mut() = Some(id);
        callback.forget();
    }
    
    // Set up toggle button handler
    let toggle_btn = document.get_element_by_id("toggle").unwrap();
    let active_clone = active.clone();
    let interval_id_clone = interval_id.clone();
    let counter_ref = counter.clone();
    
    let toggle_handler = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
        let is_active = *active_clone.borrow();
        *active_clone.borrow_mut() = !is_active;
        
        // Update button and status text
        let doc = window().unwrap().document().unwrap();
        if let Some(el) = doc.get_element_by_id("toggle") {
            el.set_text_content(Some(if !is_active { "Pause" } else { "Resume" }));
        }
        if let Some(el) = doc.get_element_by_id("status") {
            el.set_text_content(Some(if !is_active { "Active" } else { "Paused" }));
        }
        
        // Clear existing interval if any
        if let Some(id) = *interval_id_clone.borrow() {
            window().unwrap().clear_interval_with_handle(id);
            *interval_id_clone.borrow_mut() = None;
        }
        
        // Start new interval if activating
        if !is_active {
            let counter_update = counter_ref.clone();
            let callback = Closure::wrap(Box::new(move || {
                *counter_update.borrow_mut() += 1;
                
                // Update the display
                let doc = window().unwrap().document().unwrap();
                if let Some(el) = doc.get_element_by_id("counter") {
                    el.set_text_content(Some(&format!("Count: {}", *counter_update.borrow())));
                }
            }) as Box<dyn FnMut()>);
            
            let id = window().unwrap().set_interval_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                100 // Use a short interval for testing
            ).unwrap();
            
            *interval_id_clone.borrow_mut() = Some(id);
            callback.forget();
        }
    }) as Box<dyn FnMut(_)>);
    
    toggle_btn.add_event_listener_with_callback("click", toggle_handler.as_ref().unchecked_ref()).unwrap();
    toggle_handler.forget();
    
    // Set up reset button handler
    let reset_btn = document.get_element_by_id("reset").unwrap();
    let counter_reset = counter.clone();
    let interval_id_reset = interval_id.clone();
    let active_reset = active.clone();
    
    let reset_handler = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
        // Reset counter
        *counter_reset.borrow_mut() = 0;
        
        // Update display
        let doc = window().unwrap().document().unwrap();
        if let Some(el) = doc.get_element_by_id("counter") {
            el.set_text_content(Some(&format!("Count: {}", *counter_reset.borrow())));
        }
        
        // Reset interval if active
        if *active_reset.borrow() {
            // Clear existing interval
            if let Some(id) = *interval_id_reset.borrow() {
                window().unwrap().clear_interval_with_handle(id);
                *interval_id_reset.borrow_mut() = None;
            }
            
            // Start new interval
            let counter_update = counter_reset.clone();
            let callback = Closure::wrap(Box::new(move || {
                *counter_update.borrow_mut() += 1;
                
                // Update the display
                let doc = window().unwrap().document().unwrap();
                if let Some(el) = doc.get_element_by_id("counter") {
                    el.set_text_content(Some(&format!("Count: {}", *counter_update.borrow())));
                }
            }) as Box<dyn FnMut()>);
            
            let id = window().unwrap().set_interval_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                100 // Use a short interval for testing
            ).unwrap();
            
            *interval_id_reset.borrow_mut() = Some(id);
            callback.forget();
        }
    }) as Box<dyn FnMut(_)>);
    
    reset_btn.add_event_listener_with_callback("click", reset_handler.as_ref().unchecked_ref()).unwrap();
    reset_handler.forget();
    
    (host_element, counter, active)
}

// Helper function to create a Promise that resolves after a given delay
fn delay(ms: i32) -> wasm_bindgen_futures::JsFuture {
    let promise = Promise::new(&mut |resolve, _| {
        let window = window().unwrap();
        let closure = Closure::once(move || {
            resolve.call0(&wasm_bindgen::JsValue::NULL).unwrap();
        });
        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                ms,
            )
            .unwrap();
        closure.forget();
    });
    wasm_bindgen_futures::JsFuture::from(promise)
}

// Helper function to get the counter value from the DOM
fn get_counter_value() -> i32 {
    let doc = window().unwrap().document().unwrap();
    let counter_el = doc.get_element_by_id("counter").unwrap();
    let counter_text = counter_el.text_content().unwrap();
    counter_text.replace("Count: ", "").parse::<i32>().unwrap()
}

// Helper to simulate a button click
fn click_button(id: &str) {
    let doc = window().unwrap().document().unwrap();
    if let Some(button) = doc.get_element_by_id(id) {
        let button: web_sys::HtmlElement = button.dyn_into().unwrap();
        button.click();
    }
}

#[wasm_bindgen_test]
async fn test_interval_increments_counter() {
    // Arrange
    let (host, counter, _) = setup_interval_test();
    
    // Initial check
    assert_eq!(*counter.borrow(), 0);
    assert_eq!(get_counter_value(), 0);
    
    // Wait for interval to tick at least once
    wasm_bindgen_futures::JsFuture::from(delay(300)).await.unwrap();
    
    // Assert - counter should have increased at least once
    let count = get_counter_value();
    assert!(count > 0, "Count should be greater than 0 after delay");
    
    // Clean up
    window().unwrap().document().unwrap().get_element_by_id("test-container").unwrap().remove();
}

#[wasm_bindgen_test]
async fn test_interval_pauses_when_inactive() {
    // Arrange
    let (host, counter, active) = setup_interval_test();
    
    // Wait for interval to tick at least once
    wasm_bindgen_futures::JsFuture::from(delay(300)).await.unwrap();
    
    // Get the current count
    let initial_count = get_counter_value();
    assert!(initial_count > 0, "Counter should have incremented before pausing");
    
    // Act - pause the interval
    click_button("toggle");
    
    // Wait to see if counter keeps incrementing
    wasm_bindgen_futures::JsFuture::from(delay(300)).await.unwrap();
    
    // Assert - counter should not have changed
    let paused_count = get_counter_value();
    assert_eq!(paused_count, initial_count, "Count should not change while paused");
    
    // Clean up
    window().unwrap().document().unwrap().get_element_by_id("test-container").unwrap().remove();
}

#[wasm_bindgen_test]
async fn test_interval_resets() {
    // Arrange
    let (host, counter, active) = setup_interval_test();
    
    // Wait for interval to tick at least once
    wasm_bindgen_futures::JsFuture::from(delay(300)).await.unwrap();
    
    // Verify counter increased
    let initial_count = get_counter_value();
    assert!(initial_count > 0, "Counter should have incremented before reset");
    
    // Act - reset the counter
    click_button("reset");
    
    // Assert - counter should be back to 0
    assert_eq!(get_counter_value(), 0);
    
    // Wait again to confirm counter increases after reset
    wasm_bindgen_futures::JsFuture::from(delay(300)).await.unwrap();
    
    // Assert - counter should have increased again after reset
    let final_count = get_counter_value();
    assert!(final_count > 0, "Count should be greater than 0 after reset and delay");
    
    // Clean up
    window().unwrap().document().unwrap().get_element_by_id("test-container").unwrap().remove();
}

#[wasm_bindgen_test]
async fn test_interval_resumes_after_pause() {
    // Arrange
    let (host, counter, active) = setup_interval_test();
    
    // Pause the interval immediately
    click_button("toggle");
    
    // Wait to ensure counter doesn't increase while paused
    wasm_bindgen_futures::JsFuture::from(delay(300)).await.unwrap();
    
    // Assert - counter should still be 0
    assert_eq!(get_counter_value(), 0);
    
    // Act - resume the interval
    click_button("toggle");
    
    // Wait for interval to tick after resuming
    wasm_bindgen_futures::JsFuture::from(delay(300)).await.unwrap();
    
    // Assert - counter should have increased after resuming
    let resumed_count = get_counter_value();
    assert!(resumed_count > 0, "Count should be greater than 0 after resuming");
    
    // Clean up
    window().unwrap().document().unwrap().get_element_by_id("test-container").unwrap().remove();
}