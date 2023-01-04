
use io::input_handler;
use logger::log_initialize;
use terminal::terminal_initialize;

use std::{time::Duration, thread::{self}};

use game::row::Row;

use cyclic_list::{list::List};

use crate::game::{CursorPosChangeDir, CursorRotChangeDir};

mod game;
mod io;
mod logger;
mod terminal;
pub mod utils;

fn main() {

    log_initialize();    

    let _terminal = terminal_initialize();


    let mut game = game::Game::<6,10>::default();


    Row::random(&mut game.board[0]);

    let (_t, io_stream) = input_handler();
    
    loop {
        {
            let mut io_stream = io_stream.lock().unwrap();
            let io_stream = &mut *io_stream;

            while io_stream.len() > 0 {
                let event = io_stream.remove_front().unwrap();

                match event {
                    io::GameEvent::Key(char) => {
                        match char {
                            //move cursor
                            'w' => {
                                game.move_cursor(0, 1);
                            },
                            'a' => {
                                game.move_cursor(-1, 0);
                            },
                            's' => {
                                game.move_cursor(0, -1);
                            },
                            'd' => {
                                game.move_cursor(1, 0);
                            },

                            //rotate
                            'q' => {
                                game.change_cursor_rot(CursorRotChangeDir::Clockwise);
                            },
                            'e' => {
                                game.change_cursor_rot(CursorRotChangeDir::CounterClockwise);
                            },

                            //rotate
                            'Q' => {
                                game.change_cursor_length(CursorPosChangeDir::Shrink);
                            },
                            'E' => {
                                game.change_cursor_length(CursorPosChangeDir::Grow);
                            },

                            //cut and paste
                            'x' => {

                            },
                            'v' => {

                            },

                            _ => {

                            }
                        }
                    },
                }
            }
        }
        
        println!("{:?}", game);
        game.next();

        thread::sleep(Duration::from_millis(250));
    }
    

}

