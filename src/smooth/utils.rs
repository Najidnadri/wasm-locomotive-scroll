use std::{rc::Rc, cell::RefCell};

use wasm_bindgen::JsCast;
use web_sys::{WheelEvent, Element, console, HtmlElement};

use crate::{core::Core, option::{LocomotiveOption, Position}, utils::{get_translate, lerp}};

use super::SmoothScroll;


#[derive(Debug, Clone)]
pub struct Section {
    pub persistent: Option<bool>,
    pub offset: Position,
    pub limit: Position,
    pub in_view: bool,
    pub el: Element,
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

    }

    pub fn transform(el: Element, x: Option<f64>, y: Option<f64>, delay: Option<f64>) {
        let mut transform = String::new();
        let x = x.unwrap_or(0.0);
        let y = y.unwrap_or(0.0);
        if let Some(delay) = delay {
            let start = get_translate(&el).unwrap_or_else(|| {
                console::warn_1(&"cannot get 'transform' property from element".into());
                Position::new(0.0, 0.0)
            });
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
    }
}