use std::{rc::Rc, cell::RefCell};


use convert_js::ToJs;
use wasm_bindgen::JsCast;
use web_sys::Element;

use crate::{smooth::SmoothScroll, native::NativeScroll, option::LocomotiveOption, utils::instance::Instance, virtual_scroll::{VirtualScroll, VsOption}, core::Core};


#[derive(Debug, Clone)]
pub enum Scroll {
    None,
    Smooth(SmoothScroll),
    Native(NativeScroll),
}

impl Scroll {
    pub fn init(&mut self, html: Rc<RefCell<Element>>, instance: &mut Instance) {
        match self {
            Scroll::Smooth(scroll) => {
                //scroll.init(html, instance);
            },
            Scroll::Native(_scroll) => {
                //scroll.init(instance);
            },
            _ => todo!()
        }
    }

    pub fn get_option(&self) -> &LocomotiveOption {
        match self {
            Scroll::Smooth(scroll) => {
                &scroll.options
            },
            Scroll::Native(scroll) => {
                &scroll.options
            },
            _ => todo!()
        }
    }

    pub fn set_virtual_scroll(&mut self, option: VsOption) {
        match self {
            Scroll::Smooth(scroll) => {
                let vs = VirtualScroll::new(option.to_js());
                scroll.virtual_scroll = Some(vs);
            },
            _ => ()
        }
    }

    pub fn set_vs_event_listener(&self) {

        match self {
            Scroll::Smooth(scroll) => {
                let cb = scroll.vs_cb_1.clone();
                scroll.virtual_scroll.as_ref().unwrap().on(cb.as_ref().borrow().as_ref().unwrap().as_ref().unchecked_ref());
            },
            _ => ()
        }
    }
    /* 
    pub fn set_scrollbar(&mut self, scrollbar: Element, thumb: Element) {
        match self {
            Scroll::Smooth(scroll) => {
                scroll.scrollbar = Some(scrollbar);
                scroll.scrollbar_thumb = Some(thumb);
            },
            _ => {
                ()
            }
        }
    }
    */
    pub fn get_smooth(&self) -> &SmoothScroll {
        match self {
            Scroll::Smooth(scroll) => scroll,
            _ => panic!("cannot get reference to smooth scroll"),
        }
    }

    pub fn get_mut_smooth(&mut self) -> &mut SmoothScroll {
        match self {
            Scroll::Smooth(scroll) => scroll,
            _ => panic!("cannot get mutable smooth scroll")
        }
    }

    pub fn is_smooth(&self) -> bool {
        match self {
            Scroll::Smooth(_) => true,
            _ => false,
        }
    }

    pub fn is_native(&self) -> bool {
        match self {
            Scroll::Native(_) => true,
            _ => false
        }
    }
}
