use super::part;
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

    pub fn rename(&mut self, newname: &str) {
        self.name = String::from(newname);
    }
}
