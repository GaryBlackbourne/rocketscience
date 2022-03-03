use super::Executable;

use crate::models;

use super::ship_management::{DisassembleShip, EditShip, ListShips, NewShip};
use dialoguer::{theme::ColorfulTheme, Select};
use std::process::exit;

pub struct MainMenu;

impl Executable for MainMenu {
    fn execute(&self, _shipyard: &mut Vec<models::Ship>) -> Box<dyn Executable> {
        let options = &[
            "Exit",
            "New Ship",
            "List Ships",
            "Disassemble Ship",
            "Rename Ship",
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
            "Exit" => {
                println!("Bye! :)");
                exit(0);
            }
            _ => Box::new(MainMenu),
        }
    }
}
