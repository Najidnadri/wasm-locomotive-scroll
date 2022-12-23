use std::{collections::HashMap, panic, rc::Rc, cell::RefCell};

use convert_js::{__internal::JsObject, ToJs};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Element, DomRect};

use crate::{smooth::Section, option::Position};

#[derive(Clone, Debug)]
pub struct MappedEl {
    pub el: Option<Element>,
    pub target_el: Option<Element>,
    pub id: String,
    pub class: String,
    pub top: f64,
    pub bottom: f64,
    pub middle: Option<Position>,
    pub left: f64,
    pub right: f64,
    pub offset: Vec<String>,
    pub progress: Option<f64>,
    pub repeat: Option<bool>,
    pub in_view: Option<bool>,
    pub call: Option<String>,
    pub section: Option<Rc<RefCell<Section>>>,
    pub speed: Option<f64>,
    pub delay: Option<String>,
    pub position: Option<String>,
    pub direction: Option<String>,
    pub sticky: Option<String>,
}

impl Default for MappedEl {
    fn default() -> Self {
        MappedEl {
            el: None,
            target_el: None,
            id: String::new(),
            class: String::new(),
            top: 0.0,
            bottom: 0.0,
            middle: None,
            left: 0.0,
            right: 0.0,
            offset: vec![],
            progress: None,
            repeat: None,
            in_view: None,
            call: None,
            section: None,
            speed: None,
            delay: None,
            position: None,
            direction: None,
            sticky: None,
        }
    }
}



impl MappedEl {
    pub fn overwrite(&mut self, other: Self) {
        self.el = other.el;
        self.target_el = other.target_el;
        self.id = other.id;
        self.class = other.class;
        self.top = other.top;
        self.bottom = other.bottom;
        self.middle = other.middle;
        self.left = other.left;
        self.right = other.right;
        self.offset = other.offset;
        self.progress = other.progress;
        self.repeat = other.repeat;
        self.in_view = other.in_view;
        self.call = other.call;
        self.section = other.section;
        self.speed = other.speed;
        self.delay = other.delay;
        self.position = other.position;
        self.direction = other.direction;
        self.sticky = other.sticky;
    }

    pub fn to_js_value(&self) -> JsValue {
        let jsobject = JsObject::new();
        let target_el = if let Some(el) = self.target_el.clone() {
            Some(el.dyn_into::<JsValue>().unwrap())
        } else {
            None
        };
        let sections = if let Some(all_sections) = self.section.as_ref() {
            let all_sections = all_sections.borrow().to_js();
            Some(all_sections)
        } else {
            None
        };
        jsobject.set_prop(&"el".to_string(), self.el.as_ref().unwrap().dyn_ref::<JsValue>().unwrap());
        jsobject.set_prop(&"targetEl".to_string(), &target_el);
        jsobject.set_prop(&"id".to_string(), &self.id);
        jsobject.set_prop(&"class".to_string(), &self.class);
        jsobject.set_prop(&"top".to_string(), &self.top);
        jsobject.set_prop(&"bottom".to_string(), &self.bottom);
        jsobject.set_prop(&"middle".to_string(), &self.middle);
        jsobject.set_prop(&"left".to_string(), &self.left);
        jsobject.set_prop(&"right".to_string(), &self.right);
        jsobject.set_prop(&"offset".to_string(), &self.offset);
        jsobject.set_prop(&"progress".to_string(), &self.progress);
        jsobject.set_prop(&"repeat".to_string(), &self.repeat);
        jsobject.set_prop(&"inView".to_string(), &self.in_view);
        jsobject.set_prop(&"call".to_string(), &self.call);
        jsobject.set_prop(&"section".to_string(), &sections);
        jsobject.set_prop(&"delay".to_string(), &self.delay);
        jsobject.set_prop(&"position".to_string(), &self.position);
        jsobject.set_prop(&"direction".to_string(), &self.direction);
        jsobject.set_prop(&"sticky".to_string(), &self.sticky);



        jsobject.into_js_value()
    }

    pub fn hash_to_js(data: &HashMap<String, Rc<RefCell<MappedEl>>>) -> JsValue {
        let jsobject = JsObject::new();
        for (key, val) in data.iter() {
            jsobject.set_prop(&key, &val.borrow().to_js_value());
        }

        jsobject.into_js_value()
    }
}


#[derive(Clone, Debug)]
pub struct Els {
    pub data: HashMap<String, Rc<RefCell<MappedEl>>>
}

impl Els {
    pub fn new() -> Self {
        Els{data: HashMap::new()}
    }
}

pub struct ScrollToOption {
    pub offset: Option<String>,
    pub callback: Option<Rc<Box<dyn Fn()>>>,
    pub duration: Option<f64>,
    pub easing: Option<[f64; 4]>,
    pub disable_lerp: Option<bool>,
}

impl Default for ScrollToOption {
    fn default() -> Self {
        ScrollToOption {
            offset: None,
            callback: None, 
            duration: None,
            easing: None,
            disable_lerp: None,
        }
    }
}

pub enum ScrollToTarget {
    String(String),
    Element(Element),
    Num(f64),
}

impl ScrollToTarget {
    pub fn get_bounding_client_rect(&self) -> DomRect {
        match self {
            Self::Element(el) => el.get_bounding_client_rect(),
            _ => panic!("scroll to target is not an element, this is a bug. please submit an issue")
        }
    }
}