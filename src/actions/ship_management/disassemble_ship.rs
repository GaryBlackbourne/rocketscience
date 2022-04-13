use super::ListShips;
use crate::actions::MainMenu;
use crate::Executable;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;

pub struct DisassembleShip;

impl Executable for DisassembleShip {
    fn execute(&self, session: &mut crate::store::Session) -> Box<dyn Executable> {
        if session.shipyard.is_empty() {
            println!("Shipyard is empty!");
            return Box::new(MainMenu);
        }

        let ship_list: Vec<String> = session
            .shipyard
            .iter()
            .map(|ship| ship.name.clone())
            .collect();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a ship to disassemble:")
            .items(ship_list.as_slice())
            .interact()
            .unwrap();

        println!(
            "{} was disassembled!",
            session.shipyard.remove(selection).name
        );
        Box::new(ListShips)
    }
}
