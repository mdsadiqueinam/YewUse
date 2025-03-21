use std::collections::HashMap;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::window;
use yew::prelude::*;

/// Breakpoint values for different screen sizes
#[derive(Clone, Debug, PartialEq)]
pub struct Breakpoints {
    /// Custom breakpoints map with boolean values
    pub custom: HashMap<String, bool>,
    /// Extra small screens (default: < 576px)
    pub xs: bool,
    /// Small screens (default: >= 576px)
    pub sm: bool,
    /// Medium screens (default: >= 768px)
    pub md: bool,
    /// Large screens (default: >= 992px)
    pub lg: bool,
    /// Extra large screens (default: >= 1200px)
    pub xl: bool,
    /// Extra extra large screens (default: >= 1400px)
    pub xxl: bool,
}

impl Default for Breakpoints {
    fn default() -> Self {
        Self {
            custom: HashMap::new(),
            xs: false,
            sm: false,
            md: false,
            lg: false,
            xl: false,
            xxl: false,
        }
    }
}

/// Options for configuring breakpoints
#[derive(Clone, PartialEq)]
pub struct UseBreakpointsOptions {
    /// Extra small screen breakpoint in pixels
    pub xs: Option<i32>,
    /// Small screen breakpoint in pixels
    pub sm: i32,
    /// Medium screen breakpoint in pixels
    pub md: i32,
    /// Large screen breakpoint in pixels
    pub lg: i32,
    /// Extra large screen breakpoint in pixels
    pub xl: i32,
    /// Extra extra large screen breakpoint in pixels
    pub xxl: i32,
    /// Custom breakpoints mapping names to pixel values
    pub custom: HashMap<String, i32>,
}

impl Default for UseBreakpointsOptions {
    fn default() -> Self {
        Self {
            xs: None,         // Below sm
            sm: 576,          // Small devices (phones)
            md: 768,          // Medium devices (tablets)
            lg: 992,          // Large devices (desktops)
            xl: 1200,         // Extra large devices (large desktops)
            xxl: 1400,        // Extra extra large devices (larger desktops)
            custom: HashMap::new(),
        }
    }
}

/// Hook for responsive design breakpoints
///
/// # Arguments
///
/// * `options` - Optional configuration for breakpoint values
///
/// # Returns
///
/// A struct containing boolean values for each breakpoint
///
/// # Example
///
/// ```rust
/// use yewuse::browser::{use_breakpoints, UseBreakpointsOptions};
/// use std::collections::HashMap;
/// use yew::prelude::*;
///
/// #[function_component(ResponsiveLayout)]
/// fn responsive_layout() -> Html {
///     // Custom breakpoints
///     let mut custom = HashMap::new();
///     custom.insert("tablet".to_string(), 600);
///     custom.insert("desktop".to_string(), 1024);
///     
///     let options = UseBreakpointsOptions {
///         custom,
///         ..Default::default()
///     };
///     
///     let breakpoints = use_breakpoints(Some(options));
///     
///     html! {
///         <div>
///             <h2>{"Responsive Layout"}</h2>
///             <p>{"Current breakpoints:"}</p>
///             <ul>
///                 <li>{"xs (mobile): "}{breakpoints.xs.to_string()}</li>
///                 <li>{"sm (small): "}{breakpoints.sm.to_string()}</li>
///                 <li>{"md (medium): "}{breakpoints.md.to_string()}</li>
///                 <li>{"lg (large): "}{breakpoints.lg.to_string()}</li>
///                 <li>{"xl (extra large): "}{breakpoints.xl.to_string()}</li>
///                 <li>{"xxl (extra extra large): "}{breakpoints.xxl.to_string()}</li>
///             </ul>
///             
///             <p>{"Custom breakpoints:"}</p>
///             <ul>
///                 {breakpoints.custom.iter().map(|(name, active)| {
///                     html! { <li>{name}{": "}{active.to_string()}</li> }
///                 }).collect::<Html>()}
///             </ul>
///             
///             // Apply different layouts based on breakpoints
///             {if breakpoints.xs {
///                 html! { <div class="mobile-layout">{"Mobile Layout"}</div> }
///             } else if breakpoints.md {
///                 html! { <div class="tablet-layout">{"Tablet Layout"}</div> }
///             } else {
///                 html! { <div class="desktop-layout">{"Desktop Layout"}</div> }
///             }}
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_breakpoints(options: Option<UseBreakpointsOptions>) -> Breakpoints {
    let options = options.unwrap_or_default();
    let breakpoints = use_state(Breakpoints::default);
    
    use_effect_with_deps(
        move |options| {
            let window = window().unwrap();
            let breakpoints_clone = breakpoints.clone();
            
            // Create a function to check breakpoints based on window width
            let check_breakpoints = {
                let options = options.clone();
                let breakpoints_clone = breakpoints_clone.clone();
                
                move || {
                    let width = window.inner_width().unwrap().as_f64().unwrap() as i32;
                    
                    // Process standard breakpoints
                    let mut new_breakpoints = Breakpoints::default();
                    
                    // XS is active when width is less than SM
                    new_breakpoints.xs = match options.xs {
                        Some(xs_value) => width >= xs_value && width < options.sm,
                        None => width < options.sm,
                    };
                    
                    // Standard breakpoints
                    new_breakpoints.sm = width >= options.sm && width < options.md;
                    new_breakpoints.md = width >= options.md && width < options.lg;
                    new_breakpoints.lg = width >= options.lg && width < options.xl;
                    new_breakpoints.xl = width >= options.xl && width < options.xxl;
                    new_breakpoints.xxl = width >= options.xxl;
                    
                    // Process custom breakpoints
                    for (name, value) in &options.custom {
                        new_breakpoints.custom.insert(name.clone(), width >= *value);
                    }
                    
                    breakpoints_clone.set(new_breakpoints);
                }
            };
            
            // Initial check
            check_breakpoints();
            
            // Add window resize listener
            let resize_callback = {
                let check_breakpoints = check_breakpoints.clone();
                
                Closure::wrap(Box::new(move |_| {
                    check_breakpoints();
                }) as Box<dyn FnMut(_)>)
            };
            
            window
                .add_event_listener_with_callback("resize", resize_callback.as_ref().unchecked_ref())
                .unwrap();
            
            // Clean up
            move || {
                window
                    .remove_event_listener_with_callback(
                        "resize",
                        resize_callback.as_ref().unchecked_ref(),
                    )
                    .unwrap();
            }
        },
        options,
    );
    
    (*breakpoints).clone()
}