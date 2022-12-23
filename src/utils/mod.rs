use web_sys::{Element, window};

use crate::option::Position;

pub mod current_elements;
pub mod element_type;
pub mod els;
pub mod instance;
pub mod listeners;

pub fn lerp(start: f64, end: f64, amt: f64) -> f64 {
    (1.0 - amt) * start + amt * end
}


pub fn get_translate(el: &Element) -> Position {
    let style =  window().unwrap().get_computed_style(&el);

    if let Ok(Some(style)) = style {
        let transform = style.get_property_value("transform").unwrap_or_else(|_| {
            style.get_property_value("-webkit-transform").unwrap_or_else(|_| {
                style.get_property_value("-moz-transform").unwrap_or(String::new())
            })
        });
        
        if transform.starts_with("matrix3d(") {
            let parts: Vec<&str> = transform[9..transform.len()-1].split(", ").collect();
            let x = parts[12];
            let y = parts[13];
            return Position::new(x.parse::<f64>().unwrap(), y.parse::<f64>().unwrap())
        } else if transform.starts_with("matrix(") {
            let parts: Vec<&str> = transform[7..transform.len()-1].split(", ").collect();
            let x = parts[4];
            let y = parts[5];
            return Position::new(x.parse::<f64>().unwrap(), y.parse::<f64>().unwrap())
        } else {
            return Position::new(0.0, 0.0)
        }
    } else {
    }

    Position { x: 0.0, y: 0.0 }
}

pub fn get_parents(mut elem: Element) -> Vec<Element> {
    // Initialize an empty `Vec` to store the parent elements.
    let mut parents = Vec::new();

    // Loop through each parent element, pushing it to the `parents` `Vec`.
    while let Some(parent) = elem.parent_element() {
        parents.push(parent.clone());
        elem = parent;
    }

    // Return the `parents` `Vec`.
    parents
}

/* 
fn _get_translate(el: Element) -> Option<Position> {
    let window = web_sys::window()?;
    let document = window.document()?;

    let style = window.get_computed_style(&el).unwrap().unwrap();

    let webkit_transform = style.get_property_value("-webkit-transform");
    let moz_transform = style.get_property_value("-moz-transform");
    let transform = style.get_property_value("transform");

    let transform = match (webkit_transform, moz_transform, transform) {
        (Err(_), Err(_), Err(_)) => return None,
        (Err(_), Err(_), Ok(x)) | (Err(_), Ok(x), Err(_)) | (Ok(x), Err(_), Err(_)) => x,
        (Ok(x), Ok(y), Err(_)) | (Ok(y), Ok(x), Err(_)) | (Err(_), Ok(x), Ok(y)) | (Err(_), Ok(y), Ok(x)) => {
            if x.len() < y.len() {
                x
            } else {
                y
            }
        }
        (Ok(x), Ok(y), Ok(z)) => {
            if x.len() < y.len() {
                if x.len() < z.len() {
                    x
                } else {
                    z
                }
            } else if y.len() < z.len() {
                y
            } else {
                z
            }
        }
    };

    if transform.starts_with("matrix3d(") {
        let parts: Vec<&str> = transform[9..transform.len()-1].split(", ").collect();
        let x = parts[12];
        let y = parts[13];
        Some(Position::new(x.parse::<f64>().unwrap(), y.parse::<f64>().unwrap()))
    } else if transform.starts_with("matrix(") {
        let parts: Vec<&str> = transform[7..transform.len()-1].split(", ").collect();
        let x = parts[4];
        let y = parts[5];
        Some(Position::new(x.parse::<f64>().unwrap(), y.parse::<f64>().unwrap()))
    } else {
        None
    }
}
*/

/* 
let str = "matrix3d(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16)";

if str.starts_with("matrix3d(") && str.ends_with(")") {
  let str_without_parens = &str[9..str.len()-1];
  println!("{}", str_without_parens);
}
*/
