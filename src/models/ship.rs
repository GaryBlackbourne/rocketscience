use crate::models::part;
use std::vec::Vec;

pub struct Ship {
    pub name: String,
    parts: Vec<part::Part>,
}

impl Ship {
    pub fn new(name: &str) -> Ship {
        Ship {
            name: String::from(name),
            parts: Vec::new(),
        }
    }
}
