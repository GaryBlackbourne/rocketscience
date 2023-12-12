use super::{Executable, ship_management};

use crate::store::Session;
use dialoguer::{theme::ColorfulTheme, Select};

enum MainMenuState {
    Exit,
    NewShip,
    ListShips,
    DisassembleShip,
    RenameShip,
    StartGame,
}

pub struct MainMenu {
    state: MainMenuState
}

impl MainMenu {
    pub fn new() -> Self{
        MainMenu{
            state: MainMenuState::StartGame
        }
    }
    pub fn exit(&self) -> bool {
        matches!(self.state, MainMenuState::Exit)
    }
}

impl Executable for MainMenu {
    fn execute(&mut self, session: &mut Session) {
        let options = &[
            "Exit",
            "New Ship",
            "List Ships",
            "Disassemble Ship",
            "Rename Ship",
            "Start Game",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What do you want to do?")
            .default(0)
            .items(&options[..])
            .interact()
            .expect("Menu generation error!");

        self.state = match options[selection] {
            "New Ship" => MainMenuState::NewShip,
            "List Ships" => MainMenuState::ListShips,
            "Disassemble Ship" => MainMenuState::DisassembleShip,
            "Rename Ship" => MainMenuState::RenameShip,
            "Start Game" => MainMenuState::StartGame,
            "Exit" => MainMenuState::Exit,
            _ => panic!("invalid input in main menu")
        };

        match self.state {
            MainMenuState::Exit => return,
            MainMenuState::NewShip => ship_management::new_ship(session),
            MainMenuState::ListShips => ship_management::list_ship(session),
            MainMenuState::DisassembleShip => ship_management::disassemble_ship(session),
            MainMenuState::RenameShip => ship_management::edit_ship(session),
            MainMenuState::StartGame => return, // todo
        }
    }
}
