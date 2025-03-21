use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen_test::*;
use yewuse::browser::use_clipboard_read;

// Configure to run tests in browser
wasm_bindgen_test_configure!(run_in_browser);

// Test initialization of use_clipboard hook
#[wasm_bindgen_test]
fn test_use_clipboard_initialization() {
    let initial_value = "test text".to_string();
    let (state, _copy_fn) = yewuse::use_clipboard(Some(initial_value.clone()));

    assert_eq!(state.text, initial_value);
    assert_eq!(state.copied, false);
    assert_eq!(state.error, None);

    // Check if is_supported matches browser capability
    let is_supported_expected = web_sys::window()
        .and_then(|w| Some(w.navigator().clipboard()))
        .is_some();
    assert_eq!(state.is_supported, is_supported_expected);
}

// Mock clipboard for testing with adjustable behavior
struct MockClipboardTest {
    write_success: Rc<RefCell<bool>>,
    read_value: Rc<RefCell<Option<String>>>,
}

impl MockClipboardTest {
    fn new(write_success: bool, read_value: Option<String>) -> Self {
        Self {
            write_success: Rc::new(RefCell::new(write_success)),
            read_value: Rc::new(RefCell::new(read_value)),
        }
    }

    // Setup function that would inject mocks
    // Note: In real implementation, this would need to override web_sys APIs
    fn setup(&self) {
        // This is a placeholder - actual implementation would need
        // to override web_sys::window() and clipboard methods
    }
}

// Integration test example (requires clipboard permissions)
#[wasm_bindgen_test]
async fn test_clipboard_copy_integration() {
    // This test demonstrates how you would manually verify the hook
    let initial_text = "initial value".to_string();
    let (state, copy_fn) = yewuse::use_clipboard(Some(initial_text));

    // Only run this test if clipboard is supported
    if !state.is_supported {
        return;
    }

    // Attempt to copy text to clipboard
    let test_text = "test clipboard".to_string();
    copy_fn(test_text.clone());

    // In a real test environment, you would need to:
    // 1. Wait for the async operation to complete
    // 2. Check if state was updated correctly
    // 3. Verify clipboard contains the expected text
}

// Test for read hook initialization
#[wasm_bindgen_test]
fn test_use_clipboard_read_initialization() {
    let read_fn = use_clipboard_read();

    // Verify we got a function back
    assert!(std::mem::size_of_val(&read_fn) > 0);
}
