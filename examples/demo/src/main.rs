use yew::prelude::*;
use yewuse::browser::{use_clipboard, use_document_title, use_local_storage, use_window_size};
use yewuse::sensors::{use_mouse_position, use_media_query};
use yewuse::state::{use_counter, use_toggle, use_debounce};
use yewuse::ui::{use_click_outside, use_scroll};
use yewuse::utils::{use_interval};

#[function_component(App)]
fn app() -> Html {
    // Set the document title
    use_document_title("YewUse Demo".to_string());
    
    // Track window size
    let window_size = use_window_size();
    
    // Track mouse position
    let mouse_pos = use_mouse_position();
    
    // Create a counter with min, max and step
    let (count, actions) = use_counter(0, Some((Some(-5), Some(10), Some(1))));
    
    // Create a toggle state
    let (is_open, toggle, set_open) = use_toggle(false);
    
    // Create a persistent counter using local storage
    let (stored_count, set_stored_count) = use_local_storage::<i32>("stored-count", 0);
    
    // Set up a interval to update the time
    let time = use_state(|| {
        let date = js_sys::Date::new_0();
        date.to_locale_time_string("en-US")
    });
    
    let time_callback = {
        let time = time.clone();
        Callback::from(move |_| {
            let date = js_sys::Date::new_0();
            time.set(date.to_locale_time_string("en-US"));
        })
    };
    
    let _ = use_interval(time_callback, Some(1000));
    
    // Check if the screen is in mobile view
    let is_mobile = use_media_query("(max-width: 768px)");
    
    // Use clipboard integration
    let (clipboard_result, copy_fn, _) = use_clipboard();
    let text_to_copy = use_state(|| "Copy me to clipboard!".to_string());
    
    let handle_copy = {
        let text = (*text_to_copy).clone();
        Callback::from(move |_| {
            copy_fn(text.clone());
        })
    };
    
    let text_change = {
        let text_to_copy = text_to_copy.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                text_to_copy.set(target.value());
            }
        })
    };
    
    // Create a debounced search input
    let input = use_state(|| String::new());
    let debounced_input = use_debounce((*input).clone(), 500);
    
    let on_input = {
        let input = input.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                input.set(target.value());
            }
        })
    };
    
    // Use click outside for dropdown
    let dropdown_ref = use_node_ref();
    let dropdown_open = use_state(|| false);
    
    let toggle_dropdown = {
        let dropdown_open = dropdown_open.clone();
        Callback::from(move |_| {
            dropdown_open.set(!*dropdown_open);
        })
    };
    
    let close_dropdown = {
        let dropdown_open = dropdown_open.clone();
        Callback::from(move |_| {
            dropdown_open.set(false);
        })
    };
    
    use_click_outside(dropdown_ref.clone(), close_dropdown);
    
    // Track scroll position of window
    let scroll_info = use_scroll(None);

    html! {
        <div class="container">
            <h1>{"YewUse Demo"}</h1>
            <p>{"A demonstration of various hooks from the YewUse library"}</p>
            
            <div class="demo-section">
                <h2>{"Device & Browser Info"}</h2>
                <div class="info-grid">
                    <div>
                        <h3>{"Window Size"}</h3>
                        <p>{format!("Width: {}px, Height: {}px", window_size.width, window_size.height)}</p>
                    </div>
                    
                    <div>
                        <h3>{"Mouse Position"}</h3>
                        <p>{format!("X: {}, Y: {}", mouse_pos.x, mouse_pos.y)}</p>
                    </div>
                    
                    <div>
                        <h3>{"Current Time"}</h3>
                        <p>{(*time).clone()}</p>
                    </div>
                    
                    <div>
                        <h3>{"Device Type"}</h3>
                        <p>{if is_mobile { "Mobile View" } else { "Desktop View" }}</p>
                    </div>
                    
                    <div>
                        <h3>{"Scroll Position"}</h3>
                        <p>{format!("Y: {}px ({}%)", scroll_info.y, scroll_info.percentage_y.round())}</p>
                    </div>
                </div>
            </div>
            
            <div class="demo-section">
                <h2>{"State Management"}</h2>
                
                <div class="demo-row">
                    <div class="demo-card">
                        <h3>{"Counter with Limits"}</h3>
                        <p>{"Value: "}{*count}{" (Min: -5, Max: 10)"}</p>
                        <div class="button-group">
                            <button onclick={move |_| actions.decrement.emit(None)}>{"Decrement"}</button>
                            <button onclick={move |_| actions.increment.emit(None)}>{"Increment"}</button>
                            <button onclick={move |_| actions.reset.emit(())}>{"Reset"}</button>
                        </div>
                    </div>
                    
                    <div class="demo-card">
                        <h3>{"Toggle State"}</h3>
                        <p>{"Current state: "}{if *is_open { "Open" } else { "Closed" }}</p>
                        <div class="button-group">
                            <button onclick={move |_| toggle.emit(())}>{"Toggle"}</button>
                            <button onclick={move |_| set_open.emit(true)}>{"Open"}</button>
                            <button onclick={move |_| set_open.emit(false)}>{"Close"}</button>
                        </div>
                    </div>
                    
                    <div class="demo-card">
                        <h3>{"Persistent Counter"}</h3>
                        <p>{"Value: "}{*stored_count}{" (Persists on reload)"}</p>
                        <div class="button-group">
                            <button onclick={move |_| set_stored_count(*stored_count - 1)}>{"Decrement"}</button>
                            <button onclick={move |_| set_stored_count(*stored_count + 1)}>{"Increment"}</button>
                            <button onclick={move |_| set_stored_count(0)}>{"Reset"}</button>
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="demo-section">
                <h2>{"UI Interactions"}</h2>
                
                <div class="demo-row">
                    <div class="demo-card">
                        <h3>{"Clipboard Integration"}</h3>
                        <input 
                            type="text" 
                            value={(*text_to_copy).clone()} 
                            oninput={text_change}
                            placeholder="Enter text to copy"
                        />
                        <button onclick={handle_copy}>{"Copy to Clipboard"}</button>
                        <p>
                            {"Status: "}
                            {match *clipboard_result {
                                yewuse::browser::ClipboardResult::Success => "Copied!",
                                yewuse::browser::ClipboardResult::Error(_) => "Failed to copy",
                                yewuse::browser::ClipboardResult::Idle => "Ready to copy",
                            }}
                        </p>
                    </div>
                    
                    <div class="demo-card">
                        <h3>{"Debounced Input"}</h3>
                        <input 
                            type="text" 
                            value={(*input).clone()} 
                            oninput={on_input}
                            placeholder="Type to see debounce (500ms)"
                        />
                        <p>{"Current: "}{(*input).clone()}</p>
                        <p>{"Debounced: "}{debounced_input.clone()}</p>
                    </div>
                    
                    <div class="demo-card dropdown-container" ref={dropdown_ref.clone()}>
                        <h3>{"Click Outside"}</h3>
                        <button onclick={toggle_dropdown}>
                            {"Toggle Dropdown"}
                        </button>
                        
                        if *dropdown_open {
                            <div class="dropdown-content">
                                <p>{"This dropdown will close when you click outside of it"}</p>
                                <ul>
                                    <li>{"Option 1"}</li>
                                    <li>{"Option 2"}</li>
                                    <li>{"Option 3"}</li>
                                </ul>
                            </div>
                        }
                    </div>
                </div>
            </div>
            
            <footer>
                <p>{"YewUse - A collection of essential utilities for Yew applications"}</p>
                <p>{"Scroll Position: "}{scroll_info.percentage_y.round()}{"%"}</p>
            </footer>
        </div>
    }
}

#[function_component(Main)]
pub fn main() -> Html {
    html! {
        <App />
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}