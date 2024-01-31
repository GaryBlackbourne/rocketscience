use crate::store::Session;
pub trait Executable {
    fn execute(&mut self, session: &mut Session);
}

mod menu;
mod ship_building;
mod ship_management;

// Public interface of the module
pub use menu::MainMenu;
