mod actions;
mod models;

mod store {
    use crate::models;

    pub struct Session {
        pub shipyard: Vec<models::Ship>,
        current_ship: usize,
    }

    impl Session {
        pub fn new() -> Session {
            Session {
                shipyard: Vec::new(),
                current_ship: 0,
            }
        }

        pub fn current_ship(&mut self) -> &mut crate::models::Ship {
            &mut self.shipyard[self.current_ship]
        }

        pub fn next_round(&mut self) {
            if self.shipyard.len() > 0 {
                self.current_ship = (self.current_ship + 1) % self.shipyard.len();
            }
        }
    }
}

use actions::Executable;

fn main() {
    let mut session = store::Session::new();
    let mut action: Box<dyn actions::Executable> = Box::new(actions::MainMenu);

    loop {
        action = action.execute(&mut session);
    }
}
