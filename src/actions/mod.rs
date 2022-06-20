pub trait Executable {
    fn execute(&self, session: &mut crate::store::Session) -> Box<dyn Executable>;
}

mod menu;
mod ship_building;
mod ship_management;

// Public interface of the module
pub use menu::MainMenu;
