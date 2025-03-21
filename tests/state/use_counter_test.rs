use wasm_bindgen_test::*;
use yew::prelude::*;
use yewuse::state::use_counter;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{window, Document, HtmlElement};

wasm_bindgen_test_configure!(run_in_browser);

// Helper function to create and render a component for testing
fn setup_component<F, R>(render_fn: F) -> (Document, R)
where
    F: FnOnce() -> (HtmlElement, R)
{
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
    
    // Call the render function
    let (element, result) = render_fn();
    container.append_child(&element).unwrap();
    
    (document, result)
}

// Helper to create a component and return necessary elements/values
fn create_counter_test(initial: i32, min: Option<i32>, max: Option<i32>, step: Option<i32>) -> (HtmlElement, UseStateHandle<i32>, i32)
{
    let host = window().unwrap().document().unwrap().create_element("div").unwrap();
    let host_element: HtmlElement = host.dyn_into().unwrap();
    
    // Use create_portal to render our component for testing
    let count_handle = use_state(|| initial);
    let options = if min.is_some() || max.is_some() || step.is_some() {
        Some((min, max, step))
    } else {
        None
    };
    
    let (counter, actions) = {
        let (count, actions) = use_counter(initial, options);
        (count, actions)
    };
    
    // Create test content
    let content = format!(
        r#"
        <div id="counter-test">
            <span id="count">{}</span>
            <button id="increment">+</button>
            <button id="decrement">-</button>
            <button id="reset">Reset</button>
            <button id="set-5">Set to 5</button>
        </div>
        "#,
        *counter
    );
    
    host_element.set_inner_html(&content);
    
    // Set up event listeners
    let doc = window().unwrap().document().unwrap();
    if let Some(btn) = doc.get_element_by_id("increment") {
        let actions_clone = actions.clone();
        let handler = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            actions_clone.increment.emit(None);
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref()).unwrap();
        handler.forget();
    }
    
    if let Some(btn) = doc.get_element_by_id("decrement") {
        let actions_clone = actions.clone();
        let handler = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            actions_clone.decrement.emit(None);
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref()).unwrap();
        handler.forget();
    }
    
    if let Some(btn) = doc.get_element_by_id("reset") {
        let actions_clone = actions.clone();
        let handler = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            actions_clone.reset.emit(());
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref()).unwrap();
        handler.forget();
    }
    
    if let Some(btn) = doc.get_element_by_id("set-5") {
        let actions_clone = actions.clone();
        let handler = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            actions_clone.set.emit(5);
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref()).unwrap();
        handler.forget();
    }
    
    (host_element, counter, initial)
}

fn get_count_value() -> i32 {
    let doc = window().unwrap().document().unwrap();
    let count_el = doc.get_element_by_id("count").unwrap();
    count_el.text_content().unwrap().parse::<i32>().unwrap()
}

fn click_button(id: &str) {
    let doc = window().unwrap().document().unwrap();
    if let Some(button) = doc.get_element_by_id(id) {
        let button: web_sys::HtmlElement = button.dyn_into().unwrap();
        button.click();
    }
}

#[wasm_bindgen_test]
fn counter_initial_value_test() {
    let initial_value = 0;
    let (_, counter, _) = create_counter_test(initial_value, None, None, None);
    
    // Assert
    assert_eq!(*counter, initial_value);
}

#[wasm_bindgen_test]
fn counter_increment_test() {
    let initial_value = 0;
    let (_, _, _) = create_counter_test(initial_value, None, None, None);
    
    // Initial value check
    assert_eq!(get_count_value(), initial_value);
    
    // Act - click increment
    click_button("increment");
    
    // Assert
    assert_eq!(get_count_value(), 1);
    
    // Act again
    click_button("increment");
    
    // Assert
    assert_eq!(get_count_value(), 2);
}

#[wasm_bindgen_test]
fn counter_decrement_test() {
    let initial_value = 5;
    let (_, _, _) = create_counter_test(initial_value, None, None, None);
    
    // Initial value check
    assert_eq!(get_count_value(), initial_value);
    
    // Act - click decrement
    click_button("decrement");
    
    // Assert
    assert_eq!(get_count_value(), 4);
}

#[wasm_bindgen_test]
fn counter_reset_test() {
    let initial_value = 0;
    let (_, _, _) = create_counter_test(initial_value, None, None, None);
    
    // Increment first
    click_button("increment");
    click_button("increment");
    assert_eq!(get_count_value(), 2);
    
    // Act - reset
    click_button("reset");
    
    // Assert
    assert_eq!(get_count_value(), initial_value);
}

#[wasm_bindgen_test]
fn counter_set_test() {
    let initial_value = 0;
    let (_, _, _) = create_counter_test(initial_value, None, None, None);
    
    // Act - set to 5
    click_button("set-5");
    
    // Assert
    assert_eq!(get_count_value(), 5);
}

#[wasm_bindgen_test]
fn counter_respects_min_max_test() {
    // Testing with min=-5, max=5
    let initial_value = 0;
    let (_, _, _) = create_counter_test(initial_value, Some(-5), Some(5), None);
    
    // Try to increment beyond max
    for _ in 0..10 {
        click_button("increment");
    }
    
    // Should stop at max (5)
    assert_eq!(get_count_value(), 5);
    
    // Try to decrement beyond min
    for _ in 0..20 {
        click_button("decrement");
    }
    
    // Should stop at min (-5)
    assert_eq!(get_count_value(), -5);
}