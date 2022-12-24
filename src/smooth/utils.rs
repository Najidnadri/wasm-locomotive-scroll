use std::{rc::Rc, cell::RefCell, collections::HashMap};

use convert_js::{ToJs, __internal::JsObject};
use js_sys::Date;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{WheelEvent, Element, HtmlElement};

use crate::{core::Core, option::{LocomotiveOption, Position}, utils::{get_translate, lerp, els::MappedEl}};

use super::SmoothScroll;

const EL: &'static str = "el";
const IN_VIEW: &'static str = "inView";
const POSITION: &'static str = "position";
const SPEED: &'static str = "speed";
const TOP: &'static str = "top";
const BOTTOM: &'static str = "bottom";
const LEFT: &'static str = "left";
const RIGHT: &'static str = "right";
const MIDDLE: &'static str = "middle";
const STICKY: &'static str = "sticky";
const DIRECTION: &'static str = "direction";
const DELAY: &'static str = "delay";
const PERSISTENT: &'static str = "persistent";
const OFFSET: &'static str = "offset";
const LIMIT: &'static str = "limit";
const ID: &'static str = "id";


#[derive(Debug, Clone)]
pub struct Section {
    pub persistent: Option<bool>,
    pub offset: Position,
    pub limit: Position,
    pub in_view: bool,
    pub el: Element,
    pub id: String,
}

impl ToJs for Section {
    fn to_js(&self) -> JsValue {
        let jsobject = JsObject::new();

        jsobject.set_prop(&PERSISTENT, &self.persistent);
        jsobject.set_prop(&OFFSET, &self.offset);
        jsobject.set_prop(&LIMIT, &self.limit);
        jsobject.set_prop(&IN_VIEW, &self.in_view);
        jsobject.set_prop(&EL, &self.el.clone().dyn_into::<JsValue>().unwrap());
        jsobject.set_prop(&ID, &self.id);

        jsobject.into_js_value()
    }
}

#[derive(Debug, Clone)]
pub struct ParallaxElement {
    pub el: Element,
    pub in_view: bool,
    pub position: String,
    pub speed: Option<f64>,
    pub top: f64,
    pub bottom: f64,
    pub left: f64,
    pub right: f64,
    pub middle: Position,
    pub sticky: bool,
    pub direction: String,
    pub delay: Option<f64>,
}

impl ToJs for ParallaxElement {
    fn to_js(&self) -> JsValue {
        let jsobject = JsObject::new();

        jsobject.set_prop(&EL, self.el.dyn_ref::<JsValue>().unwrap());
        jsobject.set_prop(&IN_VIEW, &self.in_view);
        jsobject.set_prop(&POSITION, &self.position);
        jsobject.set_prop(&SPEED, &self.speed);
        jsobject.set_prop(&TOP, &self.top);
        jsobject.set_prop(&BOTTOM, &self.bottom);
        jsobject.set_prop(&LEFT, &self.left);
        jsobject.set_prop(&RIGHT, &self.right);
        jsobject.set_prop(&MIDDLE, &self.middle);
        jsobject.set_prop(&STICKY, &self.sticky);
        jsobject.set_prop(&DIRECTION, &self.direction);
        jsobject.set_prop(&DELAY, &self.delay);

        jsobject.into_js_value()
    }
}


#[derive(Debug, Clone)]
pub struct Sections {
    pub data: HashMap<String, Rc<RefCell<Section>>>,
}

impl Sections {
    pub fn new() -> Self {
        Self {
            data: HashMap::new()
        }
    }

    pub fn clear(&mut self) {
        self.data.clear()
    }
}

impl ToJs for Sections {
    fn to_js(&self) -> JsValue {
        let jsobject = JsObject::new();

        for (key, val) in self.data.iter() {
            let val = val.as_ref().borrow().to_js();
            jsobject.set_prop(key, &val);
        }

        jsobject.into_js_value()
    }
}

#[derive(Debug, Clone)]
pub struct ParallaxElements {
    pub data: HashMap<String, Rc<RefCell<MappedEl>>>
}

impl ParallaxElements {
    pub fn new() -> Self {
        ParallaxElements { data: HashMap::new() }
    }
}

impl ToJs for ParallaxElements {
    fn to_js(&self) -> JsValue {
        let jsobject = JsObject::new();

        for (key, val) in self.data.iter() {
            let val = val.borrow();
            jsobject.set_prop(key, &val.to_js_value());
        }
        
        jsobject.into_js_value()
    }
}











pub fn get_gesture_direction(core: Rc<RefCell<Core>>, options: &LocomotiveOption) -> String {
    let ref_core = core.as_ref().borrow();
    let ref_context = ref_core.context.as_ref().borrow();
    let context = ref_context.as_str();
    match context {
        "desktop" => {
            options.gesture_direction.clone()
        },
        "smartphone" => {
            options.smartphone.as_ref().unwrap().gesture_direction.clone()
        },
        "tablet" => {
            options.tablet.as_ref().unwrap().gesture_direction.clone()
        },
        _ => panic!("device context not supported")
    }


}

impl SmoothScroll {
    pub fn update_delta(event: WheelEvent, core: Rc<RefCell<Core>>, options: LocomotiveOption) {
        let gesture_direction = get_gesture_direction(core.clone(), &options);

        let delta = match gesture_direction.as_str() {
            "both" => event.delta_x() + event.delta_y(),
            "vertical" => event.delta_y(),
            "horizontal" => event.delta_x(),
            _ => panic!("gesture direction not supported")
        };

        {
            let core = core.as_ref().borrow();
            let mut instance = core.instance.as_ref().borrow_mut();
            let direction_axis = core.direction_axis.as_ref().borrow();
            let limit = instance.limit.clone();
            
            match *direction_axis {
                'x' => {
                    let delta_ins = instance.delta.as_mut().unwrap();
                    delta_ins.x -= delta * options.multiplier;
                    if delta_ins.x < 0.0 {
                        delta_ins.x = 0.0
                    }
                    if delta_ins.x > limit.x {
                        delta_ins.x = limit.x;
                    }
                },
                'y' => {
                    let delta_ins = instance.delta.as_mut().unwrap();
                    delta_ins.y -= delta * options.multiplier;
                    if delta_ins.y < 0.0 {
                        delta_ins.y = 0.0
                    }
                    if delta_ins.y > limit.y {
                        delta_ins.y = limit.y;
                    }
                },
                _ => panic!("direction axis not supported"),
            }
            
        }

     
    }

    pub fn start_scrolling(core: Rc<RefCell<Core>>, options: LocomotiveOption) {
        let core_ref = core.borrow();
        let scroll = core_ref.scroll.get_smooth();

        {
            *scroll.start_scroll_ts.clone().borrow_mut() = Some(Date::now());
            *scroll.is_scrolling.clone().borrow_mut() = true;
        }
        {
            SmoothScroll::check_scroll(None, core.clone(), options.clone());
        }
        {
            core_ref.html.borrow().class_list().add_1(&options.scrolling_class).unwrap();
        }

    }

    pub fn transform(el: Element, x: Option<f64>, y: Option<f64>, delay: Option<f64>) {

        let mut transform = String::new();
        let x = x.unwrap_or(0.0);
        let y = y.unwrap_or(0.0);
        if let Some(delay) = delay {
            let start = get_translate(&el);
            let lerp_x = lerp(start.x, x, delay);
            let lerp_y = lerp(start.y, y, delay);
            transform.push_str(&format!("matrix3d(1,0,0.00,0,0.00,1,0.00,0,0,0,1,0,{},{},0,1)", lerp_x, lerp_y));
        } else {
            transform.push_str(&format!("matrix3d(1,0,0.00,0,0.00,1,0.00,0,0,0,1,0,{},{},0,1)", x, y))
        }

        let style = el.dyn_ref::<HtmlElement>().unwrap().style();
        style.set_property("webkitTransform", &transform).unwrap();
        style.set_property("msTransform", &transform).unwrap();
        style.set_property("transform", &transform).unwrap();

        //let js_val = el.dyn_ref::<JsValue>().unwrap();
        //web_sys::console::log_1(js_val);
    }
}