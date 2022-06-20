use super::ShipBuilding;
use crate::store::Session;
use crate::Executable;
use coord_2d::Coord;

pub struct PrintParts;

impl Executable for PrintParts {
    fn execute(&self, session: &mut Session) -> Box<dyn Executable> {
        let ship = session.current_ship();
        let (frame_min, frame_max) = ship.get_size();

        for y in frame_min.y..=frame_max.y {
            for x in frame_min.x..=frame_max.x {
                match ship.get_part(Coord::new(x, y)) {
                    Ok(res) => print!("{}", res.get_symbol()),
                    _ => print!(" "),
                }
            }
            println!(" ");
        }

        Box::new(ShipBuilding)
    }
}
