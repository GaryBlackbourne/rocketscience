mod interface;
mod models;

mod store {
    use crate::models::Ship;

    pub struct Session {
        pub shipyard: Vec<Ship>,
        index: usize // current ship index, implements a circular buffer
    }


    impl Session {
        pub fn new() -> Session {
            Session {
                shipyard: Vec::new(),
                index: 0,
            }
        }

        pub fn current_ship(&mut self) -> Option<&mut Ship> {
            if self.shipyard.is_empty() {
                return None;
            } else {
                return Some(&mut self.shipyard[self.index])
            }
        }

        pub fn _next_round(&mut self) {
            if self.shipyard.len() > 0 {
                self.index = (self.index + 1) % self.shipyard.len();
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
