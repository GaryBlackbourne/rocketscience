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

        let _ = match self.state {
            ShipBuildingState::Finish => return,
            ShipBuildingState::InsertPart => insert_part_to_ship(session),
            ShipBuildingState::PrintCurrentDesign => print_parts(session),
        };

    }
}

pub fn insert_part_to_ship(session: &mut Session) -> Result<(), String> {
    let x: i32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Give me the X coord of the part")
        .default(0)
        .interact()
        .expect("Error while reading input");
    
    let y: i32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Give me the Y coord of the part")
        .default(0)
        .interact()
        .expect("Error while reading input");
    
    let coord = Coord::new(x, y);

    if let Some(current_ship) = session.current_ship() {
        let _ = current_ship.insert_part(coord, Part::new(String::from("Random part")));
        Ok(())
    } else {
        Err("There is no ship in the shipyard!".to_string())
    }
}

pub fn print_parts(session: &mut Session) -> Result<(), String> {
    if let Some(ship) = session.current_ship() {
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
        return Ok(());
    }
    else {
        return Err("There is no ship in shipyard".to_string());
    }
}
