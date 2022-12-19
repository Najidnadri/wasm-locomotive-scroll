use std::{collections::HashMap, panic, rc::Rc, cell::RefCell};

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
        self.left = other.left;
        self.progress = other.progress;
        self.repeat = other.repeat;
        self.call = other.call;
    }
}


#[derive(Clone, Debug)]
pub struct Els {
    pub data: HashMap<String, MappedEl>
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