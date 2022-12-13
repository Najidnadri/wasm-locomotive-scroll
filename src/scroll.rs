use std::{rc::Rc, cell::RefCell};


use web_sys::Element;

use crate::{smooth::SmoothScroll, native::NativeScroll, instance::Instance, option::LocomotiveOption};


#[derive(Debug, Clone)]
pub enum Scroll {
    Smooth(SmoothScroll),
    Native(NativeScroll),
}

impl Scroll {
    pub fn init(&mut self, html: Rc<RefCell<Element>>, instance: &mut Instance) {
        match self {
            Scroll::Smooth(scroll) => {
                scroll.init(html, instance);
            },
            Scroll::Native(_scroll) => {
                //scroll.init(instance);
            }
        }
    }

    pub fn get_option(&self) -> LocomotiveOption {
        match self {
            Scroll::Smooth(scroll) => {
                scroll.options.clone()
            },
            Scroll::Native(scroll) => {
                scroll.options.clone()
            }
        }
    }
}
