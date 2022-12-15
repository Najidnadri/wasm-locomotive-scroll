use std::collections::HashMap;

use super::els::MappedEl;

#[derive(Clone, Debug)]

pub struct CurrentElements {
    pub data: HashMap<String, MappedEl>,
}

impl CurrentElements {
    pub fn new() -> Self {
        CurrentElements { data: HashMap::new() }
    }
}