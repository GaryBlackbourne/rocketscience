use crate::actions::MainMenu;
use crate::Executable;

pub struct ListShips;

impl Executable for ListShips {
    fn execute(&self, session: &mut crate::store::Session) -> Box<dyn Executable> {
        for ship in &session.shipyard {
            println!("Name: {}", ship.name);
        }
        Box::new(MainMenu)
    }
}
