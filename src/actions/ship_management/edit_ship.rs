use super::ListShips;
use crate::actions::MainMenu;
use crate::Executable;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};

pub struct EditShip;

impl Executable for EditShip {
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
            .with_prompt("Select a ship to edit:")
            .items(ship_list.as_slice())
            .interact()
            .unwrap();

        let new_name: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("New name:")
            .interact_text()
            .unwrap();

        session.shipyard[selection].rename(new_name);

        Box::new(ListShips)
    }
}
