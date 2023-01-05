use std::{fmt::Display, mem};

use crossterm::event::{KeyEvent, KeyCode};

use crate::{model::Model, view::io::input_handler::InputQueue};

pub struct Menu<M> where M: Iterator + DoubleEndedIterator + Display + Default {
    pub title: String,
    pub menu: M,
}

impl<M> Menu<M> where M: Iterator + DoubleEndedIterator + Display + Default {
    pub fn new(title: String, start_state: M) -> Self {
        Self {
            title: title,
            menu: start_state
        }
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

        mem::replace(self, MainMenuOption::try_from(i).unwrap());
        
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

        mem::replace(self, MainMenuOption::try_from(i).unwrap());
        
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
    pub fn play() {

    }
    pub fn quit() {
        panic!("I'm just gonna terminate the application the old school way");
    }

    pub fn options() -> [&'static str; 2] {
        ["Play", "Quit"]
    }
}

impl Model<KeyEvent> for Menu<MainMenuOption> {
    fn update(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Enter => todo!(),
            KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                self.menu.next_back();
            },
            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S')  => {
                self.menu.next();
            },

            _ => {

            }
        }
    }
}