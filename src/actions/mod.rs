pub trait Executable {
    fn execute(&self, shipyard: &mut Vec<crate::models::Ship>) -> Box<dyn Executable>;
}

mod menu;
mod ship_management;

// Public interface of the module
pub use menu::MainMenu;
