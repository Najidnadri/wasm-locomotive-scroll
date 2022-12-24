
use convert_js::ToJs;
use wasm_bindgen::JsCast;

use crate::{smooth::SmoothScroll, native::NativeScroll, option::LocomotiveOption, virtual_scroll::{VirtualScroll, VsOption}};


#[derive(Debug, Clone)]
pub enum Scroll {
    None,
    Smooth(SmoothScroll),
    _Native(NativeScroll),
}

impl Scroll {

    pub fn get_option(&self) -> &LocomotiveOption {
        match self {
            Scroll::Smooth(scroll) => {
                &scroll.options
            },
            Scroll::_Native(scroll) => {
                &scroll.options
            },
            _ => todo!()
        }
    }

    pub fn get_mut_option(&mut self) -> &mut LocomotiveOption {
        match self {
            Scroll::Smooth(scroll) => &mut scroll.options,
            Scroll::_Native(_) => todo!(),
            _ => panic!()
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

    pub fn _is_native(&self) -> bool {
        match self {
            Scroll::_Native(_) => true,
            _ => false
        }
    }
}
