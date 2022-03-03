pub trait Executable {
    fn execute(&self, shipyard: &mut Vec<crate::models::Ship>) -> Box<dyn Executable>;
}

mod menu {
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
}

mod ship_management {
    use crate::actions::MainMenu;
    use crate::models::Ship;
    use crate::{models, Executable};
    use dialoguer::theme::ColorfulTheme;
    use dialoguer::{Input, Select};

    pub struct NewShip;

    impl Executable for NewShip {
        fn execute(&self, shipyard: &mut Vec<Ship>) -> Box<dyn Executable> {
            let name: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Ships name?")
                .interact_text()
                .unwrap();

            let rocket = models::Ship::new(name);
            println!("{} was created!", rocket.name);
            shipyard.push(rocket);

            Box::new(MainMenu)
        }
    }

    pub struct ListShips;

    impl Executable for ListShips {
        fn execute(&self, shipyard: &mut Vec<Ship>) -> Box<dyn Executable> {
            for ship in shipyard {
                println!("Name: {}", ship.name);
            }
            Box::new(MainMenu)
        }
    }

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
            Box::new(MainMenu)
        }
    }

    pub struct EditShip;

    impl Executable for EditShip {
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

            let new_name: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("New name:")
                .interact_text()
                .unwrap();

            shipyard[selection].rename(new_name);

            Box::new(MainMenu)
        }
    }
}

// Public interface of the module
pub use menu::MainMenu;
