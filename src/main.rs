mod models;

use dialoguer::{theme::ColorfulTheme, Input, Select};

fn main() {
    let options = &["Exit", "New Ship"];

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
            println!("{}, was created!", rocket.name);
        }
        _ => (),
    }
}
