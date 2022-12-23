use std::{rc::Rc, cell::RefCell};


use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Element, Window, window, HtmlElement, console, ScrollToOptions, ScrollBehavior};

use crate::{option::LocomotiveOption, virtual_scroll::VirtualScroll, utils::{els::{ScrollToOption, ScrollToTarget}, instance::Instance}};




#[derive(Debug, Clone)]
pub struct NativeScroll {
    pub options: LocomotiveOption,
    pub virtual_scroll: Option<VirtualScroll>,
}

impl NativeScroll {
    pub fn _new(options: LocomotiveOption, window: &Window, check_scroll_cb: Closure<dyn Fn()>) -> Self {
        
        //1
        if options.reset_native_scroll {
            let history = window.history().unwrap();
            if history.scroll_restoration().is_ok() {
                history.set_scroll_restoration(web_sys::ScrollRestoration::Manual).unwrap();
            }
            window.scroll_to_with_x_and_y(0.0, 0.0);
        }

        //2
        window.add_event_listener_with_callback_and_bool("scroll", check_scroll_cb.as_ref().unchecked_ref(), false).unwrap();
        check_scroll_cb.forget();
        //3
        /*
        if (window.smoothscrollPolyfill === undefined) {
            window.smoothscrollPolyfill = smoothscroll;
            window.smoothscrollPolyfill.polyfill();
        }
        */

        Self {
            options,
            virtual_scroll: None
        }

    }

    pub fn _scroll_to(target: ScrollToTarget, option: ScrollToOption, html: &Element, instance: Rc<RefCell<Instance>>) {
        let mut offset = match option.offset {
            Some(val) => val.parse::<f64>().unwrap().trunc(),
            None => 0.0
        };  

        let target = match target {
            ScrollToTarget::String(string) => {
                match string.as_str() {
                    "top" => {
                        html.get_bounding_client_rect().top() + offset + instance.as_ref().borrow().scroll.y
                    },
                    "bottom" => {
                        let html_el = html.dyn_ref::<HtmlElement>().unwrap();
                        html_el.offset_height() as f64 - window().unwrap().inner_height().unwrap().as_f64().unwrap() + offset
                    },
                    any => {
                        let el = window().unwrap().document().unwrap().query_selector(any).unwrap();
                        if let Some(el) = el {
                            el.get_bounding_client_rect().top() + offset + instance.as_ref().borrow().scroll.y
                        } else {
                            console::warn_1(&"[target] parameter is not valid".into());
                            panic!()
                        }
                    }
                }
            },
            ScrollToTarget::Element(el) => {
                el.get_bounding_client_rect().top() + offset + instance.as_ref().borrow().scroll.y
            },
            ScrollToTarget::Num(num) => {
                num.trunc() + offset
            }
        };

        offset = target;

        if let Some(callback) = option.callback {
            if _is_target_reached(offset) {
                callback();
                return;
            } else {
                let onscroll: Rc<RefCell<Option<Closure<dyn FnMut() >>>> = Rc::new(RefCell::new(None));
                let onscroll2 = onscroll.clone();
                
                *onscroll.borrow_mut() = Some(Closure::new(move || {
                    let callback = callback.clone();
                    if _is_target_reached(offset) {
                        let cb = onscroll2.borrow();
                        let cb = cb.as_ref().unwrap().as_ref().unchecked_ref();
                        window().unwrap().remove_event_listener_with_callback("scroll", cb).unwrap();

                        callback.as_ref()();
                    }
                }));

                let cb = onscroll.borrow();
                let cb = cb.as_ref().unwrap().as_ref().unchecked_ref();
                window().unwrap().add_event_listener_with_callback("scroll", cb).unwrap();

            }
        }

        let scroll_to_behaviour = match option.duration {
            Some(val) => if val == 0.0 {ScrollBehavior::Auto} else {ScrollBehavior::Smooth},
            None => ScrollBehavior::Auto
        };
        let mut scroll_to_options = ScrollToOptions::new();
        scroll_to_options.top(offset);
        scroll_to_options.behavior(scroll_to_behaviour);
        window().unwrap().scroll_to_with_scroll_to_options(&scroll_to_options);

    }

}



fn _is_target_reached(offset: f64) -> bool {
    window().unwrap().page_y_offset().unwrap().trunc() == offset.trunc()
}