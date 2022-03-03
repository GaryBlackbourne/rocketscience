use super::ListShips;
use crate::actions::MainMenu;
use crate::models::Ship;
use crate::Executable;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;

pub struct DisassembleShip;

impl Executable for DisassembleShip {
    fn execute(&self, shipyard: &mut Vec<Ship>) -> Box<dyn Executable> {
        if shipyard.is_empty() {
            println!("Shipyard is empty!");
            return Box::new(MainMenu);
        }

        let ship_list: Vec<String> = shipyard.iter().map(|ship| ship.name.clone()).collect();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a ship to disassemble:")
            .items(ship_list.as_slice())
            .interact()
            .unwrap();

        println!("{} was disassembled!", shipyard.remove(selection).name);
        Box::new(ListShips)
    }
}
