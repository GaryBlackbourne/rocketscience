use crate::actions::ship_building::ShipBuilding;
use crate::models::Part;
use crate::store::Session;
use crate::Executable;
use coord_2d::Coord;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;

pub struct InsertPartToShip;

impl Executable for InsertPartToShip {
    fn execute(&self, session: &mut Session) -> Box<dyn Executable> {
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

        Box::new(ShipBuilding)
    }
}
