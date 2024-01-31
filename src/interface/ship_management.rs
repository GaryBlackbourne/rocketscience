use crate::Session;
use dialoguer::{theme::ColorfulTheme, Input, Select};

pub fn new_ship(session: &mut Session) -> Result<(), String> {
    use crate::models::Ship;
    if let Ok(name) = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Ships name?")
        .interact_text()
    {
        let rocket = Ship::new(name);
        println!("{} was created!", rocket.name);
        session.shipyard.push(rocket);
        Ok(())
    } else {
        Err("Could not read input".to_string())
    }
}

pub fn edit_ship(session: &mut Session) -> Result<(), String> {
    if session.shipyard.is_empty() {
        return Err("Shipyard is empty".to_string());
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
        .expect("Error while reading list input!");

    let new_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("New name:")
        .interact_text()
        .expect("Error while reading input!"); // ez errort ad üres stringre is ? 

    session.shipyard[selection].rename(new_name);
    Ok(())
}

pub fn list_ship(session: &mut Session) -> Result<(), String> {
    for ship in &session.shipyard {
        println!("Name: {}", ship.name);
    }

    Ok(())
}

pub fn disassemble_ship(session: &mut Session) -> Result<(), String> {
    if session.shipyard.is_empty() {
        return Err("Shipyard is empty!".to_string());
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
        .expect("Error while reading list input!");

    println!(
        "{} was disassembled!",
        session.shipyard.remove(selection).name
    );
    Ok(())
}
