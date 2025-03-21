// YewUse - Collection of essential utilities for Yew applications
//! # YewUse
//! 
//! YewUse is a collection of essential utilities for Yew applications, inspired by VueUse.
//! It provides a set of composable functions that enable you to build reactive 
//! and robust web applications more effectively.
//!
//! ## Categories
//!
//! YewUse is organized into several categories of utilities:
//!
//! - **Browser**: Utilities for interacting with browser APIs
//! - **Sensors**: Hooks for detecting user input and device capabilities
//! - **State**: Utilities for state management
//! - **UI**: UI-related utilities and effect hooks
//! - **Utils**: General utilities and helpers
//!
//! ## Examples
//!
//! ```rust
//! use yewuse::browser::use_local_storage;
//! use yewuse::state::use_counter;
//! use yew::prelude::*;
//!
//! #[function_component(Counter)]
//! fn counter() -> Html {
//!     // Persistent counter state that saves to localStorage
//!     let (count, set_count) = use_local_storage::<i32>("counter", 0);
//!     
//!     // Counter with built-in increment/decrement functions
//!     let (value, actions) = use_counter(0, Some((Some(-10), Some(10), Some(1))));
//!     
//!     html! {
//!         <div>
//!             <div>
//!                 <h2>{"Persistent Counter"}</h2>
//!                 <p>{"Count: "}{*count}</p>
//!                 <button onclick={move |_| set_count(*count + 1)}>{"Increment"}</button>
//!             </div>
//!             
//!             <div>
//!                 <h2>{"Basic Counter with Min/Max"}</h2>
//!                 <p>{"Value: "}{*value}</p>
//!                 <button onclick={move |_| actions.decrement.emit(None)}>{"Decrement"}</button>
//!                 <button onclick={move |_| actions.increment.emit(None)}>{"Increment"}</button>
//!                 <button onclick={move |_| actions.reset.emit(())}>{"Reset"}</button>
//!             </div>
//!         </div>
//!     }
//! }
//! ```
#![warn(missing_docs)]

pub mod browser;
pub mod sensors;
pub mod state;
pub mod ui;
pub mod utils;

// Re-export most commonly used hooks for convenience
pub use browser::{use_clipboard, use_document_title, use_local_storage, use_window_size, use_preferred_dark, use_favicon, use_idle, use_breakpoints};
pub use sensors::{use_mouse_position, use_mouse_position_simple, use_media_query, use_network_state, use_keyboard, use_geolocation};
pub use state::{use_counter, use_toggle, use_debounce, use_color_mode};
pub use ui::{use_click_outside, use_scroll, use_scroll_lock, use_element_size, use_intersection_observer};
pub use utils::{use_async, use_interval, use_timeout, use_websocket};
