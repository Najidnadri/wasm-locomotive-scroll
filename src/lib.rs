mod option;
mod native;
mod smooth;
mod core;
mod scroll;
mod virtual_scroll;
mod smooth_scroll;
mod utils;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue, JsCast};
pub use web_sys::*;


use std::{panic, cell::RefCell, rc::Rc};

use crate::core::Core;
use option::LocomotiveOption;
use scroll::Scroll;
//use virtual_scroll::{VirtualScroll, VsOption};
use web_sys::{console, window};

use crate::utils::element_type::ElementType;


pub const LEFT: u32 = 37;
pub const UP: u32 = 38;
pub const RIGHT: u32 = 39;
pub const DOWN: u32 = 40;
pub const SPACE: u32 = 32;
pub const TAB: u32 = 9;
pub const PAGEUP: u32 = 33;
pub const PAGEDOWN: u32 = 34;
pub const HOME: u32 = 36;
pub const END: u32 = 35;





#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Smooth {
    core: Rc<RefCell<Core>>,
}

#[wasm_bindgen]
impl Smooth {
    #[wasm_bindgen(constructor)]
    pub fn new(options: JsValue) -> Self {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        console::log_2(&"before default: ".into(), &options);
        let mut options: LocomotiveOption = serde_wasm_bindgen::from_value(options).unwrap();
        let el = window().unwrap().document().unwrap().query_selector(&options.query).unwrap().unwrap();
        options.el = ElementType::from_element(el);
        console::log_1(&format!("options after default: {:?}", options).into());

        //warnings
        if !options.smooth && options.direction == "horizontal".to_string() {
            console::warn_1(&"'Smooth: false' & 'horizontal' direction are not yet compatible".into());
        }
        if let Some(tablet) = &options.tablet {
            if !tablet.smooth && tablet.direction == "horizontal".to_string() {
                console::warn_1(&"'Smooth: false' & 'horizontal' direction are not yet compatible(Tablet)".into());
            }
        }
        if let Some(smarthphone) = &options.smartphone {
            if !smarthphone.smooth && smarthphone.direction == "horizontal".to_string() {
                console::warn_1(&"'Smooth: false' & 'horizontal' direction are not yet compatible(Smartphone)".into());
            }
        }

        //Check mobile and tablet
        options.check_mobile();
        options.check_tablet();

        //core
        let core = Core::new(options.clone());


        //Smooth
        let mut smooth = Smooth {core};

        Core::check_scroll_callback(smooth.core.clone());
        Core::check_resize_callback(smooth.core.clone());
        Core::check_event_callback(smooth.core.clone());
        {
            let window = window().unwrap();
            let check_resize_cb = Core::get_check_resize(smooth.core.clone());
            let check_resize_cb = check_resize_cb.borrow();
            let check_resize_cb = check_resize_cb.as_ref().unwrap();
            window.add_event_listener_with_callback_and_bool("resize", check_resize_cb.as_ref().unchecked_ref(), false).unwrap();  
        }
        
        let smooth_dbg = format!("{:?}", smooth);
        console::log_1(&smooth_dbg.into());

        //INIT
        smooth.init();

        let smooth_dbg = format!("{:?}", smooth);
        console::log_1(&smooth_dbg.into());

        smooth
    }


    fn init(&mut self) {

        //init core

        Core::init(self.core.clone());
    
        if let Ok(hash) = window().unwrap().location().hash() {
            //Get the hash without the `#`
            let id = hash.replace("#", "");

            //if element found, scroll to it.
            let target = window().unwrap().document().unwrap().get_element_by_id(&id);
            if let Some(_el) = target {
                //self.core.scroll_to(Some(el), None);
            }
        }
    }
 
}














































/* 
pub struct Smooth {
    pub options: LocomotiveOption,
    pub scroll: Option<Scroll>,
    pub el: ElementType,
    pub name: String,
    pub offset: [f64; 2],
    pub repeat: bool,
    pub smooth: bool,
    pub init_position: Position,
    pub direction: String,
    pub gesture_direction: String,
    pub reload_on_context_change: bool,
    pub lerp: f64,
    pub class: String,
    pub scroll_bar_container: bool,
    pub scroll_bar_class: String,
    pub scrolling_class: String,
    pub dragging_class: String,
    pub smooth_class: String,
    pub init_class: String,
    pub get_speed: bool,
    pub get_direction: bool,
    pub scroll_from_anywhere: bool,
    pub multiplier: f64,
    pub firefox_multiplier: f64,
    pub touch_multiplier: f64,
    pub reset_native_scroll: bool,
    pub tablet: Option<Tablet>,
    pub smartphone: Option<Smartphone>,
}


impl Smooth {
    pub fn new(options: LocomotiveOption) -> Self {

        //init default options
        let mut default_option = LocomotiveOption::default();

        //overwrite default, priotize on given options
        default_option.overwrite(options);
        let mut options = default_option;

        //Display some warnings 
        if !options.smooth && options.direction == "horizontal".to_string() {
            console::warn_1(&"'Smooth: false' & 'horizontal' direction are not yet compatible".into());
        }
        if let Some(tablet) = &options.tablet {
            if !tablet.smooth && tablet.direction == "horizontal".to_string() {
                console::warn_1(&"'Smooth: false' & 'horizontal' direction are not yet compatible(Tablet)".into());
            }
        }
        if let Some(smarthphone) = &options.smartphone {
            if !smarthphone.smooth && smarthphone.direction == "horizontal".to_string() {
                console::warn_1(&"'Smooth: false' & 'horizontal' direction are not yet compatible(Smartphone)".into());
            }
        }

        // Extra settings on Smooth, equal to `fn init()` on the official doc
        options.check_mobile();
        options.check_tablet();
        

        let mut smooth = Smooth::from_locomotive_opt(options);

        smooth.init();
        smooth
    }

}


impl Smooth {
    fn from_locomotive_opt(opt: LocomotiveOption) -> Self {
        Smooth {
            options: opt.clone(),
            scroll: None,
            el: opt.el,
            name: opt.name,
            offset: opt.offset,
            repeat: opt.repeat,
            smooth: opt.smooth,
            init_position: opt.init_position,
            direction: opt.direction,
            gesture_direction: opt.gesture_direction,
            reload_on_context_change: opt.reload_on_context_change,
            lerp: opt.lerp,
            class: opt.class,
            scroll_bar_container: opt.scroll_bar_container,
            scroll_bar_class: opt.scroll_bar_class,
            scrolling_class: opt.scrolling_class,
            dragging_class: opt.dragging_class,
            smooth_class: opt.smooth_class,
            init_class: opt.init_class,
            get_speed: opt.get_speed,
            get_direction: opt.get_direction,
            scroll_from_anywhere: opt.scroll_from_anywhere,
            multiplier: opt.multiplier,
            firefox_multiplier: opt.firefox_multiplier,
            touch_multiplier: opt.touch_multiplier,
            reset_native_scroll: opt.reset_native_scroll,
            tablet: opt.tablet,
            smartphone: opt.smartphone
        }
    }

    fn init(&mut self) {

        //Setup the `Scroll` Field
        if (self.smooth && !self.options.is_mobile)  || 
            (self.tablet.as_ref().unwrap().smooth && self.options.is_tablet) ||
            (self.smartphone.as_ref().unwrap().smooth && self.options.is_mobile && !self.options.is_tablet) 
            {
            self.scroll = Some(Scroll::Smooth(SmoothScroll::new(&self.options)));
        } else {
            self.scroll = Some(Scroll::Native(NativeScroll::new(&self.options)));
        }

        self.scroll.as_mut().unwrap().init();

        if let Ok(hash) = window().unwrap().location().hash() {
            //Get the hash without the `#`
            let id = &hash[1..];

            //if element found, scroll to it.
            let target = window().unwrap().document().unwrap().get_element_by_id(id);
            if let Some(el) = target {
                self.scroll.as_ref().unwrap().scroll_to(Some(el), None);
            }
        }
    }
}

type CoreBox = Rc<RefCell<Core>>;


pub struct Smooth2 {
    pub options: LocomotiveOption,
    pub core: CoreBox,
}


impl Smooth2 {
    pub fn new(opt: Option<LocomotiveOption>) -> Self {
        let mut default_option = LocomotiveOption::default();
        if let Some(option) = opt {
            default_option.overwrite(option);
        }
        let options = default_option;
        Smooth2 {
            options: options.clone(),
            core: new_core(options)
        }
    }
}

*/