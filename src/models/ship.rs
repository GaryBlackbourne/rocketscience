use super::Part;
use coord_2d::Coord;
use std::collections::HashMap;

pub struct Ship {
    pub name: String,
    grid: HashMap<Coord, Part>,
}

impl Ship {
    pub fn new(name: String) -> Ship {
        Ship {
            name,
            grid: HashMap::from([(Coord::new(0, 0), Part::new(String::from("Bridge")))]),
        }
    }

    pub fn rename(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn insert_part(&mut self, coord: Coord, part: Part) -> Result<&Part, &'static str> {
        if self.grid.contains_key(&coord) {
            return Err("Coord is taken");
        };

        self.grid.insert(coord, part);

        Ok(self.grid.get(&coord).unwrap())
    }
}
