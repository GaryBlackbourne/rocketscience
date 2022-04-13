use super::Executable;

use super::ship_management::{DisassembleShip, EditShip, ListShips, NewShip};
use crate::models::Part;
use crate::store::Session;
use coord_2d::Coord;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::process::exit;

pub struct MainMenu;

impl Executable for MainMenu {
    fn execute(&self, _session: &mut crate::store::Session) -> Box<dyn Executable> {
        let options = &[
            "Exit",
            "New Ship",
            "List Ships",
            "Disassemble Ship",
            "Rename Ship",
            "Start Game",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What do you want to do?")
            .default(0)
            .items(&options[..])
            .interact()
            .unwrap();

        match options[selection] {
            "New Ship" => Box::new(NewShip),
            "List Ships" => Box::new(ListShips),
            "Disassemble Ship" => Box::new(DisassembleShip),
            "Rename Ship" => Box::new(EditShip),
            "Start Game" => Box::new(ShipBuilding),
            "Exit" => {
                println!("Bye! :)");
                exit(0);
            }
            _ => Box::new(MainMenu),
        }
    }
}

struct ShipBuilding;

impl Executable for ShipBuilding {
    fn execute(&self, _session: &mut crate::store::Session) -> Box<dyn Executable> {
        let options = &["Finish", "Insert Part"];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Build your ship!")
            .default(0)
            .items(&options[..])
            .interact()
            .unwrap();

        match options[selection] {
            "Insert Part" => Box::new(InsertPartToShip),
            "Finish" => Box::new(MainMenu),
            _ => Box::new(ShipBuilding),
        }
    }
}

struct InsertPartToShip;

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
