use crate::actions::MainMenu;
use crate::models::Ship;
use crate::Executable;

pub struct ListShips;

impl Executable for ListShips {
    fn execute(&self, shipyard: &mut Vec<Ship>) -> Box<dyn Executable> {
        for ship in shipyard {
            println!("Name: {}", ship.name);
        }
        Box::new(MainMenu)
    }
}
