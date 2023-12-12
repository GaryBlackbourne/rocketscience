use dialoguer::{theme::ColorfulTheme, Input, Select};
use crate::Session;

pub fn new_ship(session: &mut Session) {
    use crate::models::Ship;
    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Ships name?")
        .interact_text()
        .expect("could not parse ship name");
    
    let rocket = Ship::new(name);
    println!("{} was created!", rocket.name)
;
    session.shipyard.push(rocket);
}

pub fn edit_ship(session: &mut Session) {
    if session.shipyard.is_empty() {
        println!("Shipyard is empty!");
        return;
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
}

pub fn list_ship(session: &mut Session) {
    for ship in &session.shipyard {
        println!("Name: {}", ship.name);
    }
}

pub fn disassemble_ship(session: &mut Session) {
    if session.shipyard.is_empty() {
        println!("Shipyard is empty!");
        return;
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
}
