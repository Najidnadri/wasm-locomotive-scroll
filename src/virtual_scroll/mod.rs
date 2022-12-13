use convert_js::ToJs;
use js_sys::Function;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::window;

use crate::{option::LocomotiveOption, element_type::ElementType};

 
#[wasm_bindgen(module= "/js/virtual-scroll/index.js")]
extern "C" {
    #[derive(Debug, Clone)]
    pub type VirtualScroll;

    #[wasm_bindgen(constructor)]
    pub fn new(option: JsValue) -> VirtualScroll;

    #[wasm_bindgen(method)]
    pub fn on(this: &VirtualScroll, callback: Function);
}


#[derive(Clone, ToJs)]
pub struct VsOption {
    pub el: JsValue,
    pub mouse_multiplier: f64,
    pub firefox_multiplier: f64,
    pub touch_multiplier: f64,
    pub use_keyboard: bool,
    pub passive: bool,
}


impl VsOption {
    pub fn _new(options: &LocomotiveOption) -> Self {
        let window = window().unwrap();

        let element: JsValue = match options.scroll_from_anywhere {
            true => {
                let document = window.document().unwrap();
                AsRef::<JsValue>::as_ref(&document).clone()
            },
            false => {
                match &options.el {
                    ElementType::Document(doc) => {
                        AsRef::<JsValue>::as_ref(doc).clone()
                    },
                    ElementType::Element(el) => {
                        AsRef::<JsValue>::as_ref(el).clone()
                    }
                }
            }
        };

        let mouse_multiplier = match window.navigator().platform().unwrap().find("Win") {
            Some(_) => 1.0,
            None => 0.4
        };

        Self {
            el: element,
            mouse_multiplier,
            firefox_multiplier: options.firefox_multiplier,
            touch_multiplier: options.touch_multiplier,
            use_keyboard: false,
            passive: true
        }
        
    }
}
