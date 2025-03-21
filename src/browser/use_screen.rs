use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{Document, Element};
use yew::prelude::*;
use crate::utils::use_event_listener;

/// Options for the useScreen hook
#[derive(Properties, PartialEq, Clone)]
pub struct UseScreenOptions {
    /// The target element for fullscreen. If not provided, document.documentElement is used
    pub target: Option<Element>,
}

/// State and controls for fullscreen mode
#[derive(Clone)]
pub struct ScreenControls {
    /// Whether the element is currently in fullscreen mode
    pub is_fullscreen: bool,
    /// Function to enter fullscreen mode
    pub enter: Callback<()>,
    /// Function to exit fullscreen mode
    pub exit: Callback<()>,
    /// Function to toggle fullscreen mode
    pub toggle: Callback<()>,
}

/// Hook to detect fullscreen state and control it
///
/// # Example
///
/// ```rust
/// use yew::prelude::*;
/// use web_sys::HtmlElement;
/// use yewuse::browser::use_screen;
///
/// #[function_component(FullscreenExample)]
/// fn fullscreen_example() -> Html {
///     let div_ref = use_node_ref();
///     
///     let target = div_ref.cast::<HtmlElement>(); 
///     let options = use_screen::UseScreenOptions { target };
///     
///     let controls = use_screen(options);
///     
///     html! {
///         <div ref={div_ref}>
///             <p>{"Is fullscreen: "}{controls.is_fullscreen.to_string()}</p>
///             <button onclick={move |_| controls.toggle.emit(())}>
///                 {"Toggle Fullscreen"}
///             </button>
///         </div>
///     }
/// }
/// ```
pub fn use_screen(options: UseScreenOptions) -> ScreenControls {
    let is_fullscreen = use_state(|| false);
    let target_ref = use_mut_ref(|| options.target.clone());
    
    let document = web_sys::window()
        .expect("Window should exist")
        .document()
        .expect("Document should exist");
    
    let get_target = {
        let document = document.clone();
        move || -> Element {
            if let Some(target) = target_ref.borrow().clone() {
                target
            } else {
                document.document_element().expect("Document element should exist")
            }
        }
    };

    // Update fullscreen state when it changes
    {
        let is_fullscreen = is_fullscreen.clone();
        use_effect_with_deps(
            move |_| {
                let document = web_sys::window()
                    .expect("Window should exist")
                    .document()
                    .expect("Document should exist");

                let on_fullscreen_change = Closure::wrap(Box::new(move || {
                    let is_full = document.fullscreen();
                    is_fullscreen.set(is_full);
                }) as Box<dyn FnMut()>);

                use_event_listener(
                    document.into(),
                    "fullscreenchange",
                    on_fullscreen_change.into_js_value().unchecked_into(),
                );

                || ()
            },
            (),
        );
    }

    // Create control functions
    let enter = {
        let get_target = get_target.clone();
        Callback::from(move |_| {
            let target = get_target();
            let _ = target.request_fullscreen();
        })
    };

    let exit = {
        let document = document.clone();
        Callback::from(move |_| {
            let _ = document.exit_fullscreen();
        })
    };

    let toggle = {
        let is_fullscreen = *is_fullscreen;
        let enter = enter.clone();
        let exit = exit.clone();
        Callback::from(move |_| {
            if is_fullscreen {
                exit.emit(());
            } else {
                enter.emit(());
            }
        })
    };

    ScreenControls {
        is_fullscreen: *is_fullscreen,
        enter,
        exit,
        toggle,
    }
}