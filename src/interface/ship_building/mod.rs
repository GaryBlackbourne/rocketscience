use crate::{interface::Executable, store::Session, models::Part};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use coord_2d::Coord;

enum ShipBuildingState{
    Finish,
    InsertPart,
    PrintCurrentDesign,
}
struct ShipBuilding{
    state: ShipBuildingState
}

impl Executable for ShipBuilding {
    fn execute(&mut self, session: &mut Session) {
        let options = &["Finish", "Insert Part", "Print current design"];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Build your ship!")
            .default(0)
            .items(&options[..])
            .interact()
            .unwrap();

        self.state = match options[selection] {
            "Insert Part" => ShipBuildingState::InsertPart,
            "Print current design" => ShipBuildingState::PrintCurrentDesign,
            "Finish" => ShipBuildingState::Finish,
            _ => panic!("Invalid option"),
        };

        match self.state {
            ShipBuildingState::Finish => return,
            ShipBuildingState::InsertPart => insert_part_to_ship(session),
            ShipBuildingState::PrintCurrentDesign => print_parts(session),
        }

    }
}

pub fn insert_part_to_ship(session: &mut Session) {
        let x: i32 = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Give me the X coord of the part")
            .default(0)
            .interact()
            .unwrap();

        let y: i32 = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Give me the Y coord of the part")
            .default(0)
            .interact()
            .unwrap();

        let coord = Coord::new(x, y);

        let result = session
            .current_ship()
            .insert_part(coord, Part::new(String::from("Random part")));

        match result {
            Ok(_) => println!("Inserted successfully!"),
            Err(msg) => println!("{}", msg),
        }
}

pub fn print_parts(session: &mut Session) {
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
}
