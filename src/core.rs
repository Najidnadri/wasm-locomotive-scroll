use std::{rc::Rc, cell::RefCell, collections::HashMap};

use js_sys::Function;
use wasm_bindgen::{prelude::{Closure, wasm_bindgen}, JsCast};
use web_sys::{window, Element, console, NodeList, Event};

use crate::{option::{LocomotiveOption, Position, Tablet, Smartphone}, Scroll, els::{Els, MappedEl}, element_type::ElementType};

#[derive(Clone, Debug)]

pub struct CurrentElements {
    pub data: HashMap<String, MappedEl>,
}

impl CurrentElements {
    pub fn new() -> Self {
        CurrentElements { data: HashMap::new() }
    }
}

#[derive(Clone, Debug)]
pub struct Listeners {
    
}

impl Listeners {
    pub fn new() -> Listeners {
        Listeners {  }
    }
}

#[derive(Clone, Debug)]
pub struct Instance {
    pub scroll: Position,
    pub limit: Position,
    pub current_elements: CurrentElements,
    pub direction: Option<String>,
    pub speed: Option<f64>,
}

impl Instance {
    fn new(current_elements: CurrentElements, get_direction: bool) -> Self {
        let mut speed = Some(0.);
        let direction = None;
        match get_direction {
            true => (),
            false => {
                speed = None;
            },
        };
        Instance { scroll: Position{x: 0., y: 0.}, limit: Position{x: 0., y:0.}, current_elements, direction, speed }
    }
}


#[derive(Clone, Debug)]
pub struct Core {
    // Core items
    pub namespace: String,
    pub html: Element,
    pub window_height: f64,
    pub window_width: f64,
    pub window_middle: Position,
    pub els: Els,
    pub current_elements: CurrentElements,
    pub listeners: Listeners,
    pub has_scroll_ticking: bool,
    pub has_call_event_set: bool,
    pub check_scroll: Option<Function>,
    pub check_resize: Option<Function>,
    pub check_event: Option<Function>,
    pub instance: Instance,
    pub context: String,
    pub direction_axis: char,
    pub resize_tick: bool,
    pub scroll_to_els: Option<NodeList>,
    pub scroll: Option<Scroll>,
    
    //Locomotive Options
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
    pub is_tablet: bool,
    pub smartphone: Option<Smartphone>,
    pub is_mobile: bool,
}

impl From<LocomotiveOption> for Core {
    fn from(mut opt: LocomotiveOption) -> Self {
        let window = window().unwrap();
        let height = window.inner_height().unwrap().as_f64().unwrap();
        let width = window.inner_width().unwrap().as_f64().unwrap();
        let current_elements = CurrentElements::new();
        let context = match opt.is_mobile {
            true => {
                match opt.is_tablet {
                    true => "tablet".to_string(),
                    false => "smartphone".to_string(),
                }
            },
            false => {
                "desktop".to_string()
            }
        };
        let direction_axis = if opt.direction == "horiontal".clone() {
            'x'
        } else {
            'y'
        };

        if opt.is_mobile {
            if context == "tablet".to_string() {
                opt.direction = opt.tablet.as_ref().unwrap().direction.clone();
            } else if context == "smartphone".to_string() {
                opt.direction = opt.smartphone.as_ref().unwrap().direction.clone();
            } 
        }

        Self {
            namespace: "locomotive".to_string(),
            html: window.document().unwrap().document_element().unwrap(),
            window_height: height,
            window_width: width,
            window_middle: Position { x: width / 2., y: height / 2. },
            els: Els::new(),
            current_elements: current_elements.clone(),
            listeners: Listeners::new(),
            has_scroll_ticking: false,
            has_call_event_set: false,
            check_scroll: None,
            check_resize: None,
            check_event: None,  
            instance: Instance::new(current_elements, opt.get_direction.clone()),
            context,
            direction_axis,
            resize_tick: false,
            scroll_to_els: None,
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
            is_tablet: opt.is_tablet,
            smartphone: opt.smartphone,
            is_mobile: opt.is_mobile,
        }
    }
}


pub fn new_core(options: LocomotiveOption) -> Rc<RefCell<Core>> {
    //init default options
    let mut default_option = LocomotiveOption::default();

    //overwrite default, priotize on given options
    default_option.overwrite(options);
    let options = default_option;

    let core = Core::from(options);
    let _ = core.html.class_list().add_1(&core.init_class).unwrap();

    let core = Rc::new(RefCell::new(core));
    let core1 = core.clone();

    let check_resize_cb = Closure::wrap(Box::new(move || {
        let core = core.clone();
        let resize_tick = core.as_ref().clone().into_inner().resize_tick;
        if !resize_tick {
            core.as_ref().borrow_mut().resize_tick = true;

            let callback = Closure::wrap(Box::new(move || {
                let core = core.clone();
                _resize();
                console::log_1(&"224".into());
                core.as_ref().borrow_mut().resize_tick = false;
                console::log_1(&"225".into());
            }) as Box<dyn Fn()>);

            let _ = window().unwrap().request_animation_frame(callback.as_ref().unchecked_ref()).unwrap();
            callback.forget();
        }
    }) as Box<dyn Fn()>);


    let _ = window().unwrap().add_event_listener_with_callback_and_bool("resize", check_resize_cb.as_ref().unchecked_ref(), false).unwrap();
    check_resize_cb.forget();
    
    core1
}


type CoreType = Rc<RefCell<Core>>;

fn _resize() {

}

pub fn init_core(core: CoreType) {
    init_core_events(core);
}

fn init_core_events(core: CoreType) {
    {   
        let elements = {
            console::log_1(&"258".into());
            match &core.as_ref().borrow().el {
                ElementType::Document(doc) => {
                    console::log_1(&"261".into());
                    doc.query_selector_all(&format!("[data-{}-to]", core.as_ref().borrow().name)).unwrap()
                    
                },
                ElementType::Element(el) => {
                    el.query_selector_all(&format!("[data-{}-to]", core.as_ref().borrow().name)).unwrap()
                }
            }
        };
        console::log_1(&"271".into());
        core.as_ref().borrow_mut().scroll_to_els = Some(elements);
        console::log_1(&"273".into());
    }

    {   
        let core1 = core.clone();
        let set_scroll_to_callback = Closure::wrap(Box::new(move |e: Event| {
            e.prevent_default();
            let core = core.clone();
            let core = core.as_ref().borrow();

            let element = e.current_target().unwrap().dyn_into::<Element>().unwrap();
            let mut attr = element.get_attribute(&format!("data-{}-href", core.name));
            if attr.is_none() {
                attr = element.get_attribute("href");
            }
            /* 
            match &core.scroll {
                Some(scroll) => {
                    scroll.scroll_to(None, attr, None);
                },
                Some(scroll) => {
                    scroll.scroll_to(None, attr, None);
                }
                None => ()
            }
            */

        }) as Box<dyn Fn(Event)>);

        let node_list = core1.as_ref().borrow().scroll_to_els.as_ref().unwrap().length();
    
        if node_list != 0 {
            for index in 0 .. node_list - 1 {
                let el = core1.as_ref().borrow().scroll_to_els.as_ref().unwrap().get(index).unwrap();
                let _ = el.add_event_listener_with_callback_and_bool("click", set_scroll_to_callback.as_ref().unchecked_ref(), false).unwrap();
            }
        }
    }
}








