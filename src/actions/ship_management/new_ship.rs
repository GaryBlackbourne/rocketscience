use super::ListShips;
use crate::{models, Executable};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;

pub struct NewShip;

impl Executable for NewShip {
    fn execute(&self, session: &mut crate::store::Session) -> Box<dyn Executable> {
        let name: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Ships name?")
            .interact_text()
            .unwrap();

        let rocket = models::Ship::new(name);
        println!("{} was created!", rocket.name);
        session.shipyard.push(rocket);

        Box::new(ListShips)
    }
}
