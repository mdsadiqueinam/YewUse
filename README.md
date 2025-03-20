# YewUse

YewUse is a collection of essential utilities for Yew applications, inspired by [VueUse](https://vueuse.org/) for Vue.js. It provides a set of composable functions that make building Yew applications more efficient and enjoyable.

## Features

- 🧰 **Browser API Wrappers**: Ready-to-use hooks for localStorage, windowSize, clipboard, and more
- 📡 **Sensors**: Monitor mouse, window, network status, and device capabilities
- 🧠 **State Management**: Advanced state utilities including counter, toggle, debounce, and persistent state
- 🧩 **UI Utilities**: Practical UI interactions like click-outside detection, intersection observers, and scroll tracking
- ⚙️ **General Utilities**: Common patterns for intervals, timeouts, and async operations

## Installation

Add YewUse to your Cargo.toml:

```toml
[dependencies]
yew = "0.20.0"
yewuse = "0.1.0"
```

## Usage

```rust
use yew::prelude::*;
use yewuse::browser::use_window_size;
use yewuse::state::use_counter;

#[function_component(App)]
fn app() -> Html {
    // Track window size with automatic reactivity
    let window_size = use_window_size();
    
    // Create a counter with min, max and step
    let (count, actions) = use_counter(0, Some((Some(-5), Some(10), Some(1))));
    
    html! {
        <div>
            <p>{"Window size: "}{window_size.width}{"x"}{window_size.height}</p>
            
            <p>{"Count: "}{*count}</p>
            <button onclick={move |_| actions.decrement.emit(None)}>{"Decrement"}</button>
            <button onclick={move |_| actions.increment.emit(None)}>{"Increment"}</button>
            <button onclick={move |_| actions.reset.emit(())}>{"Reset"}</button>
        </div>
    }
}
```

## Categories

### Browser

- `use_clipboard`: Copy and paste with the clipboard API
- `use_document_title`: Reactive document title
- `use_local_storage`: Persist state in localStorage
- `use_session_storage`: Persist state in sessionStorage
- `use_window_size`: Reactive window dimensions

### Sensors

- `use_device_orientation`: Track device orientation
- `use_media_query`: Reactive media queries
- `use_mouse_position`: Track mouse position
- `use_network_state`: Network status and connection information

### State

- `use_counter`: Extensible counter with min/max/step
- `use_debounce`: Debounce changing values
- `use_local_state`: Reducer-pattern state management
- `use_previous`: Track previous state values
- `use_toggle`: Boolean toggle with convenient API

### UI

- `use_click_outside`: Detect clicks outside an element
- `use_element_size`: Track element size and position
- `use_intersection_observer`: Visibility detection for elements
- `use_scroll`: Track element or window scroll position

### Utils

- `use_async`: Handle async operations with loading/error states
- `use_interval`: Self-clearing intervals
- `use_timeout`: Self-clearing timeouts
- Various DOM utility functions

## Demo

Check out the full demo in the `examples/demo` directory to see all features in action.

## License

MIT