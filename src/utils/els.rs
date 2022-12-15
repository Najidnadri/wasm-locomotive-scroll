use std::{collections::HashMap, panic, rc::Rc};

use web_sys::{Element, DomRect};

#[derive(Clone, Debug)]
pub struct MappedEl {
    pub el: Element,
    pub target_el: Element,
    pub id: String,
    pub class: String,
    pub top: f64,
    pub bottom: f64,
    pub left: f64,
    pub right: f64,
    pub offset: Vec<String>,
    pub progress: f64,
    pub repeat: bool,
    pub in_view: bool,
    pub call: Option<String>,
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