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

    pub fn get_size(&self) -> (Coord, Coord) {
        let mut min_coord = Coord::new(0, 0);
        let mut max_coord = Coord::new(0, 0);

        for grid_coord in self.grid.keys() {
            min_coord = min_coord.pairwise_min(*grid_coord);
            max_coord = max_coord.pairwise_max(*grid_coord);
        }

        (min_coord, max_coord)
    }

    pub fn get_part(&self, coord: Coord) -> Result<&Part, &'static str> {
        if !self.grid.contains_key(&coord) {
            return Err("Coord is empty");
        };

        Ok(self.grid.get(&coord).unwrap())
    }
}
