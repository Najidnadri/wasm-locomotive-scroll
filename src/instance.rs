use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

use crate::{option::Position, core::CurrentElements};

#[derive(Clone, Debug)]
pub struct Instance {
    pub scroll: Position,
    pub limit: Position,
    pub delta: Option<Position>,
    pub current_elements: CurrentElements,
    pub direction: Option<String>,
    pub speed: Option<f64>,
}

impl Instance {
    pub fn new(document_element: &Element, current_elements: CurrentElements) -> Self {
        let html_el = document_element.dyn_ref::<HtmlElement>().unwrap();
        let offset_height = html_el.offset_height();
        let offset_width = html_el.offset_width();
        Instance { scroll: Position{x: 0., y:0.}, limit: Position{x: offset_width as f64, y: offset_height as f64}, delta: None,  current_elements, direction: None, speed: None }
    }

    pub fn set_delta(&mut self, position: Position) {
        self.delta = Some(position);
    }

    pub fn set_scroll(&mut self, position: Position) {
        self.scroll = position
    }

    pub fn get_limit(&self, direction_axis: char) -> f64 {
        match direction_axis {
            'x' => self.limit.x,
            'y' => self.limit.y,
            _ => panic!("direction axis not supported")
        }
    }

    pub fn get_delta(&self, direction_axis: char) -> f64 {
        match direction_axis {
            'x' => self.delta.as_ref().unwrap().x,
            'y' => self.delta.as_ref().unwrap().y,
            _ => panic!("direction axis not supported")
        }
    }

    pub fn change_delta_with_limit(&mut self, direction_axis: char, operation: &str, other_val: f64) {
        match direction_axis {
            'x' => {
                match operation {
                    "minus" => {
                        self.delta.as_mut().unwrap().x -= other_val;
                    },
                    "plus" => {
                        self.delta.as_mut().unwrap().x += other_val;
                    }
                    _ => panic!("math operation is not supported")
                }
            },
            'y' => {
                match operation {
                    "minus" => {
                        self.delta.as_mut().unwrap().y -= other_val;
                    },
                    "plus" => {
                        self.delta.as_mut().unwrap().y += other_val;
                    }
                    _ => panic!("math operation is not supported")
                }
            },
            _ => {
                panic!("direction axis not supported");
            }
        }
    }

    pub fn update_delta(&mut self, direction_axis: char, val: f64) {
        match direction_axis {
            'x' => self.delta.as_mut().unwrap().x = val,
            'y' => self.delta.as_mut().unwrap().y = val,
            _ => panic!("direction axis not supported")
        }
    }
}