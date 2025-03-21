use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::window;
use yew::prelude::*;
use crate::browser::{use_local_storage, use_preferred_dark};

/// Available color modes
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ColorMode {
    /// Light mode
    Light,
    /// Dark mode
    Dark,
    /// Automatically use system preference
    Auto,
}

impl Default for ColorMode {
    fn default() -> Self {
        Self::Auto
    }
}

/// Hook for managing color mode with system preference detection and persistence
///
/// # Arguments
///
/// * `storage_key` - Key to use for storing preference in localStorage (optional)
/// * `default_mode` - Default color mode (optional)
///
/// # Returns
///
/// A tuple containing:
/// - Current color mode setting (Light, Dark, or Auto)
/// - Boolean indicating if dark mode is active
/// - Function to set the color mode
/// - Function to toggle between light and dark mode
///
/// # Example
///
/// ```rust
/// use yewuse::state::{use_color_mode, ColorMode};
/// use yew::prelude::*;
///
/// #[function_component(ThemeManager)]
/// fn theme_manager() -> Html {
///     let (color_mode, is_dark, set_mode, toggle) = use_color_mode(
///         Some("theme-preference".to_string()),
///         Some(ColorMode::Auto)
///     );
///     
///     // Apply theme class to the document
///     {
///         let is_dark = *is_dark;
///         use_effect_with_deps(
///             move |_| {
///                 let document = web_sys::window().unwrap().document().unwrap();
///                 let element = document.document_element().unwrap();
///                 
///                 if is_dark {
///                     element.set_class_name("dark-theme");
///                 } else {
///                     element.set_class_name("light-theme");
///                 }
///                 || {}
///             },
///             is_dark,
///         );
///     }
///     
///     html! {
///         <div>
///             <h3>{"Theme Settings"}</h3>
///             <p>{"Current mode: "}{format!("{:?}", color_mode)}</p>
///             <p>{"Dark mode active: "}{is_dark.to_string()}</p>
///             
///             <div>
///                 <button onclick={move |_| set_mode.emit(ColorMode::Light)}>{"Light"}</button>
///                 <button onclick={move |_| set_mode.emit(ColorMode::Dark)}>{"Dark"}</button>
///                 <button onclick={move |_| set_mode.emit(ColorMode::Auto)}>{"Auto"}</button>
///                 <button onclick={move |_| toggle.emit(())}>{"Toggle"}</button>
///             </div>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_color_mode(
    storage_key: Option<String>,
    default_mode: Option<ColorMode>,
) -> (
    ColorMode,               // current color mode
    UseStateHandle<bool>,    // is dark mode active
    Callback<ColorMode>,     // set color mode
    Callback<()>,            // toggle between light/dark
) {
    let storage_key = storage_key.unwrap_or_else(|| "color-mode".to_string());
    let default_mode = default_mode.unwrap_or_default();
    
    // Get system preference
    let system_prefers_dark = use_preferred_dark();
    
    // Store color mode in localStorage
    let (mode, set_mode) = use_local_storage::<ColorMode>(&storage_key, default_mode);
    
    // Track actual dark mode state based on preference and system
    let is_dark = use_state(|| {
        match *mode {
            ColorMode::Light => false,
            ColorMode::Dark => true,
            ColorMode::Auto => *system_prefers_dark,
        }
    });
    
    // Update dark mode state when preferences change
    {
        let is_dark = is_dark.clone();
        
        use_effect_with_deps(
            move |(mode, system_prefers_dark)| {
                let is_dark_value = match **mode {
                    ColorMode::Light => false,
                    ColorMode::Dark => true,
                    ColorMode::Auto => **system_prefers_dark,
                };
                
                is_dark.set(is_dark_value);
                || {}
            },
            (mode.clone(), system_prefers_dark.clone()),
        );
    }
    
    // Set color mode function
    let set_color_mode = Callback::from(move |mode: ColorMode| {
        set_mode(mode);
    });
    
    // Toggle function
    let toggle = {
        let mode = mode.clone();
        let set_mode = set_mode.clone();
        
        Callback::from(move |_| {
            let new_mode = match *mode {
                ColorMode::Light => ColorMode::Dark,
                ColorMode::Dark => ColorMode::Light,
                ColorMode::Auto => {
                    if *system_prefers_dark {
                        ColorMode::Light
                    } else {
                        ColorMode::Dark
                    }
                }
            };
            
            set_mode(new_mode);
        })
    };
    
    ((*mode).clone(), is_dark, set_color_mode, toggle)
}