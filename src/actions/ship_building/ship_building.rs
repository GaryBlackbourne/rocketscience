use super::InsertPartToShip;
use crate::actions::MainMenu;
use crate::store::Session;
use crate::Executable;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;

pub struct ShipBuilding;

impl Executable for ShipBuilding {
    fn execute(&self, _session: &mut Session) -> Box<dyn Executable> {
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
