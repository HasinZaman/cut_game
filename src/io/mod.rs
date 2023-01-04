use std::{thread::{self, JoinHandle}, sync::{Arc, Mutex}, rc::Rc, fmt::Display};

use crossterm::event::{self, Event, KeyEvent};

use cyclic_list::{CyclicList, list::List};

use crate::game;

type EventList = CyclicList<10, GameEvent, false>;

#[derive(Debug, Clone, Copy)]
pub enum GameEvent {
    Key(char)
}

impl Display for GameEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for GameEvent {
    fn default() -> Self {
        GameEvent::Key('\0')
    }
}

pub fn input_handler() -> (JoinHandle<()>, Arc<Mutex<EventList>>) {
    let io_stream = Arc::new(
        Mutex::new(
            EventList::default()
        )
    );

    let thread_io_stream = io_stream.clone();

    let t = thread::spawn(move || {
        let io_stream = thread_io_stream.clone();

        loop {
            let event = event::read();

            if let Err(_err) = event {
                continue;
            }

            let event = event.unwrap();

            match event {
                Event::FocusGained => {
                    //play game
                    todo!()
                },
                Event::FocusLost => {
                    //pause game
                    todo!()
                },
                Event::Key(event) => {
                    key_event(event, &mut *io_stream.lock().unwrap());
                },
                Event::Mouse(_)  | Event::Paste(_) => {
                    //ignore
                },
                Event::Resize(_, _) => {
                    //resize screen
                },
            }
        }
    });

    (t, io_stream)
}

fn key_event(event: KeyEvent, io_stream: &mut EventList) {
    match event.code {
        //movement
        event::KeyCode::Char('w') | event::KeyCode::Char('W') => {
            let _ = io_stream.push(GameEvent::Key('w'));
        },
        event::KeyCode::Char('a') | event::KeyCode::Char('A') => {
            let _ = io_stream.push(GameEvent::Key('a'));
        },
        event::KeyCode::Char('s') | event::KeyCode::Char('S') => {
            let _ = io_stream.push(GameEvent::Key('s'));
        },
        event::KeyCode::Char('d') | event::KeyCode::Char('D') => {
            let _ = io_stream.push(GameEvent::Key('d'));
        },

        //scaling
        event::KeyCode::Char('Q') => {
            let _ = io_stream.push(GameEvent::Key('Q'));
        },
        event::KeyCode::Char('E') => {
            let _ = io_stream.push(GameEvent::Key('E'));
        }

        //rotating
        event::KeyCode::Char('q') => {
            let _ = io_stream.push(GameEvent::Key('q'));
        },
        event::KeyCode::Char('e') => {
            let _ = io_stream.push(GameEvent::Key('e'));
        },
        
        //cut and past
        event::KeyCode::Char('X') => {
            let _ = io_stream.push(GameEvent::Key('x'));
        },
        event::KeyCode::Char('V') => {
            let _ = io_stream.push(GameEvent::Key('v'));
        },

        _ => {

        }
    }
}