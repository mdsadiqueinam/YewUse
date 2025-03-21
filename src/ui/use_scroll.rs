use wasm_bindgen::JsCast;
use web_sys::{window, Element, Event};
use yew::hook;

#[derive(Clone, Debug)]
pub struct ScrollPosition {
    pub x: f64,
    pub y: f64,
}

impl Default for ScrollPosition {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }
}

#[hook]
pub fn use_scroll(element: Option<Element>) -> ScrollPosition {
    let position = yew::use_state(ScrollPosition::default);

    {
        let position = position.clone();
        yew::use_effect_with_deps(
            move |(element,)| {
                let target = element.clone().unwrap_or_else(|| {
                    window()
                        .expect("no window")
                        .document()
                        .expect("no document")
                        .document_element()
                        .expect("no document element")
                });

                let callback = {
                    let position = position.clone();
                    let target = target.clone();
                    move |_: Event| {
                        let x = target.scroll_left() as f64;
                        let y = target.scroll_top() as f64;
                        position.set(ScrollPosition { x, y });
                    }
                };

                let closure = wasm_bindgen::closure::Closure::wrap(
                    Box::new(callback) as Box<dyn FnMut(_)>
                );

                target
                    .add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
                    .unwrap();

                // Set initial position
                position.set(ScrollPosition {
                    x: target.scroll_left() as f64,
                    y: target.scroll_top() as f64,
                });

                move || {
                    target
                        .remove_event_listener_with_callback(
                            "scroll",
                            closure.as_ref().unchecked_ref(),
                        )
                        .unwrap();
                }
            },
            (element,),
        );
    }

    (*position).clone()
}

#[hook]
pub fn use_window_scroll() -> ScrollPosition {
    use_scroll(None)
}