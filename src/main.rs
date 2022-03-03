mod actions;
mod models;

use actions::Executable;

fn main() {
    let mut shipyard: Vec<models::Ship> = Vec::new();
    let mut action: Box<dyn actions::Executable> = Box::new(actions::MainMenu);

    loop {
        action = action.execute(&mut shipyard);
    }
}
