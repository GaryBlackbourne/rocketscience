mod interface;
mod models;

mod store {
    use crate::models::Ship;

    pub struct Session {
        pub shipyard: Vec<Ship>,
        current_ship: usize,
    }


    impl Session {
        pub fn new() -> Session {
            Session {
                shipyard: Vec::new(),
                current_ship: 0,
            }
        }

        pub fn current_ship(&mut self) -> &mut Ship {
            &mut self.shipyard[self.current_ship]
        }

        pub fn _next_round(&mut self) {
            if self.shipyard.len() > 0 {
                self.current_ship = (self.current_ship + 1) % self.shipyard.len();
            }
        }

    }
}

use interface::{Executable, MainMenu};
use store::Session;

fn main() -> Result<(), ()> {

    let mut session = Session::new();
    let mut menu = MainMenu::new();

    loop {
        menu.execute(&mut session);
        if menu.exit() {
            return Ok(());
        }
    }
}
