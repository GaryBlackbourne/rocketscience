mod models;

use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::borrow::Borrow;

fn main() {
    let mut shipyard = Vec::new();

    let options = &["Exit", "New Ship", "List Ships", "Disassemble ship"];

    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What do you want to do?")
            .default(0)
            .items(&options[..])
            .interact()
            .unwrap();

        match options[selection] {
            "New Ship" => {
                let name: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Ships name?")
                    .interact_text()
                    .unwrap();

                let rocket = models::Ship::new(name);
                println!("{} was created!", rocket.name);
                shipyard.push(rocket);
            }
            "List Ships" => {
                for ship in &shipyard {
                    println!("Name: {}", ship.name);
                }
            }
            "Disassemble ship" => {
                if shipyard.is_empty() {
                    println!("Shipyard is empty!");
                    continue;
                }

                let ship_list: Vec<String> =
                    shipyard.iter().map(|ship| ship.name.clone()).collect();

                let selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select a ship to disassemble:")
                    .items(ship_list.as_slice())
                    .interact()
                    .unwrap();

                println!("{} was disassembled!", shipyard.remove(selection).name);
            }
            _ => {
                println!("Bye! :)");
                break;
            }
        }
    }
}
