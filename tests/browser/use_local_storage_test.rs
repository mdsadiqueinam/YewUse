use wasm_bindgen_test::*;
use yew::prelude::*;
use yewuse::browser::use_local_storage;
use gloo::storage::{LocalStorage, Storage};
use serde::{Serialize, Deserialize};
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{window, Document, HtmlElement};
use std::cell::RefCell;
use std::rc::Rc;

wasm_bindgen_test_configure!(run_in_browser);

// Test struct that will be stored in localStorage
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct TestUser {
    id: u32,
    name: String,
}

// Helper function to setup a test component
fn setup_local_storage_test<T: Clone + Serialize + DeserializeOwned + 'static + std::fmt::Display>(
    key: &str,
    default_value: T
) -> (HtmlElement, UseStateHandle<T>, Callback<T>)
{
    // Clear storage before test
    let _ = LocalStorage::delete(key);
    
    let host = window().unwrap().document().unwrap().create_element("div").unwrap();
    let host_element: HtmlElement = host.dyn_into().unwrap();
    
    // Use local storage hook
    let (value, set_value) = use_local_storage::<T>(key, default_value);
    
    // Create test content
    let content = format!(
        r#"
        <div id="local-storage-test">
            <span id="value">{}</span>
            <button id="increment">Increment</button>
            <button id="set-specific">Set Specific</button>
            <button id="reset">Reset</button>
        </div>
        "#,
        *value
    );
    
    host_element.set_inner_html(&content);
    
    (host_element, value, set_value)
}

// Helper to get value from storage test element
fn get_displayed_value() -> String {
    let doc = window().unwrap().document().unwrap();
    doc.get_element_by_id("value")
        .unwrap()
        .text_content()
        .unwrap()
}

// Special setup for User object test
fn setup_user_test(key: &str) -> (HtmlElement, UseStateHandle<TestUser>, Callback<TestUser>) {
    // Clear storage before test
    let _ = LocalStorage::delete(key);
    
    let default_user = TestUser { id: 1, name: "Default".to_string() };
    
    let host = window().unwrap().document().unwrap().create_element("div").unwrap();
    let host_element: HtmlElement = host.dyn_into().unwrap();
    
    // Use local storage hook
    let (user, set_user) = use_local_storage::<TestUser>(key, default_user);
    
    // Create test content
    let content = format!(
        r#"
        <div id="local-storage-test">
            <span id="user-id">{}</span>
            <span id="user-name">{}</span>
            <button id="update-name">Update Name</button>
            <button id="update-id">Update ID</button>
        </div>
        "#,
        user.id,
        user.name
    );
    
    host_element.set_inner_html(&content);
    
    // Setup click handlers
    let doc = window().unwrap().document().unwrap();
    
    // Update name button
    if let Some(btn) = doc.get_element_by_id("update-name") {
        let user_clone = user.clone();
        let set_user_clone = set_user.clone();
        
        let handler = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            let mut new_user = (*user_clone).clone();
            new_user.name = "Updated".to_string();
            set_user_clone.emit(new_user);
        }) as Box<dyn FnMut(_)>);
        
        btn.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref()).unwrap();
        handler.forget();
    }
    
    // Update ID button
    if let Some(btn) = doc.get_element_by_id("update-id") {
        let user_clone = user.clone();
        let set_user_clone = set_user.clone();
        
        let handler = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            let mut new_user = (*user_clone).clone();
            new_user.id = 99;
            set_user_clone.emit(new_user);
        }) as Box<dyn FnMut(_)>);
        
        btn.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref()).unwrap();
        handler.forget();
    }
    
    (host_element, user, set_user)
}

// Helper to get user values
fn get_user_values() -> (u32, String) {
    let doc = window().unwrap().document().unwrap();
    let id = doc.get_element_by_id("user-id")
        .unwrap()
        .text_content()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    
    let name = doc.get_element_by_id("user-name")
        .unwrap()
        .text_content()
        .unwrap();
    
    (id, name)
}

// Helper to click a button
fn click_button(id: &str) {
    let doc = window().unwrap().document().unwrap();
    if let Some(button) = doc.get_element_by_id(id) {
        let button: web_sys::HtmlElement = button.dyn_into().unwrap();
        button.click();
    }
}

#[wasm_bindgen_test]
fn local_storage_initial_value_test() {
    // Arrange - clear storage and set up component
    let key = "test-counter";
    let default_value = 0;
    let _ = LocalStorage::delete(key);
    
    // Act - create component with default value
    let (_, value, _) = setup_local_storage_test(key, default_value);
    
    // Assert - should be initialized with default value
    assert_eq!(*value, default_value);
    assert_eq!(get_displayed_value(), "0");
}

#[wasm_bindgen_test]
fn local_storage_persistence_test() {
    // Arrange - set a value in localStorage before rendering
    let key = "test-counter";
    let stored_value = 42;
    let _ = LocalStorage::delete(key);
    LocalStorage::set(key, &stored_value).unwrap();
    
    // Act - create component
    let (_, value, _) = setup_local_storage_test(key, 0);
    
    // Assert - should read from localStorage
    assert_eq!(*value, stored_value);
    assert_eq!(get_displayed_value(), "42");
}

#[wasm_bindgen_test]
fn local_storage_update_test() {
    // Arrange
    let key = "test-counter";
    let (_, value, set_value) = setup_local_storage_test(key, 0);
    
    // Initial check
    assert_eq!(*value, 0);
    
    // Act - update value
    set_value.emit(5);
    
    // Assert - should update both state and localStorage
    let stored_value: i32 = LocalStorage::get(key).unwrap();
    assert_eq!(stored_value, 5);
}

#[wasm_bindgen_test]
fn local_storage_object_test() {
    // Arrange
    let key = "test-user";
    let (_, user, _) = setup_user_test(key);
    
    // Initial check
    assert_eq!(user.id, 1);
    assert_eq!(user.name, "Default");
    
    let (id, name) = get_user_values();
    assert_eq!(id, 1);
    assert_eq!(name, "Default");
    
    // Act - update name
    click_button("update-name");
    
    // Assert
    let (id, name) = get_user_values();
    assert_eq!(id, 1);
    assert_eq!(name, "Updated");
    
    // Verify localStorage was updated
    let stored_user: TestUser = LocalStorage::get(key).unwrap();
    assert_eq!(stored_user.name, "Updated");
    
    // Act - update id
    click_button("update-id");
    
    // Assert
    let (id, name) = get_user_values();
    assert_eq!(id, 99);
    assert_eq!(name, "Updated");
    
    // Verify localStorage was updated
    let stored_user: TestUser = LocalStorage::get(key).unwrap();
    assert_eq!(stored_user.id, 99);
}

#[wasm_bindgen_test]
fn local_storage_object_persistence_test() {
    // Arrange - set a complex object in localStorage
    let key = "test-user";
    let pre_stored_user = TestUser {
        id: 123,
        name: "PreStored".to_string(),
    };
    LocalStorage::set(key, &pre_stored_user).unwrap();
    
    // Act - create component
    let (_, user, _) = setup_user_test(key);
    
    // Assert - should load the object from localStorage
    assert_eq!(user.id, 123);
    assert_eq!(user.name, "PreStored");
    
    let (id, name) = get_user_values();
    assert_eq!(id, 123);
    assert_eq!(name, "PreStored");
}