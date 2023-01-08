use std::{fmt::Display};

use crossterm::event::{KeyEvent, KeyCode};
use cyclic_list::List;

use crate::{model::Model, presenter::menu::MenuCommandQueue};

pub struct Menu<M> where M: Iterator + DoubleEndedIterator + Display + Default {
    pub title: String,
    pub menu: M,
    pub menu_commands: MenuCommandQueue,
}

impl<M> Menu<M> where M: Iterator + DoubleEndedIterator + Display + Default {
    pub fn new(title: String, start_state: M) -> Self {
        let mut menu_commands = MenuCommandQueue::default();

        let _ = menu_commands.push(MenuCommand::UpdateView);
        Self {
            title: title,
            menu: start_state,
            menu_commands,
        }
    }
}


#[derive(Debug, Clone)]
pub enum MenuCommand {
    Select(String),
    UpdateView
}
impl Default for MenuCommand {
    fn default() -> Self {
        MenuCommand::UpdateView
    }
}

#[derive(Clone, Copy)]
pub enum MainMenuOption {
    Play = 0isize,
    Quit
}

impl Iterator for MainMenuOption {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        let i = (*self as isize + 1) % 2;

        *self = MainMenuOption::try_from(i).unwrap();
        
        Some(*self)
    }
}

impl TryFrom<isize> for MainMenuOption {
    type Error = ();

    fn try_from(value: isize) -> Result<Self, ()> {
        match value {
            0isize => Ok(MainMenuOption::Play),
            1isize => Ok(MainMenuOption::Quit),
            _ => Err(()),
        }
    }

}

impl DoubleEndedIterator for MainMenuOption {
    fn next_back(&mut self) -> Option<Self::Item> {
        let i = {
            let mut tmp = *self as isize - 1;

            if tmp < 0 {
                tmp = 1;
            }

            tmp
        };

        *self = MainMenuOption::try_from(i).unwrap();
        
        Some(*self)
    }
}

impl Display for MainMenuOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MainMenuOption::Play => write!(f, "PLAY"),
            MainMenuOption::Quit => write!(f, "QUIT"),
        }
    }
}

impl Default for MainMenuOption {
    fn default() -> Self {
        MainMenuOption::Play
    }
}

impl Menu<MainMenuOption> {
    pub fn play(&mut self) {
        let _ = self.menu_commands.push(MenuCommand::Select(String::from("Play")));
    }
    pub fn quit(&self) {
        panic!("I'm just gonna terminate the application the old school way");
    }

    pub fn options() -> [&'static str; 2] {
        ["Play", "Quit"]
    }
}

impl Model<KeyEvent, MenuCommandQueue> for Menu<MainMenuOption> {
    fn update_self(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Enter => {
                match self.menu {
                    MainMenuOption::Play => self.play(),
                    MainMenuOption::Quit => self.quit(),
                }
            },
            KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                self.menu.next_back();
                let _ = self.menu_commands.push(MenuCommand::UpdateView);
            },
            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S')  => {
                self.menu.next();
                let _ = self.menu_commands.push(MenuCommand::UpdateView);
            },

            _ => {

            }
        }
    }

    fn update_presenter(&mut self) -> MenuCommandQueue {
        let mut commands = MenuCommandQueue::default();

        while let Some(val) = self.menu_commands.remove_front() {
            let _ = commands.push(val.clone());
        }

        commands
    }
}