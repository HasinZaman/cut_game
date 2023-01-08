use std::{sync::{mpsc::{self, Receiver, Sender}, Mutex, Arc, RwLock}, thread::{self, JoinHandle}, time::Duration, cmp::{min, max}};

use crossterm::event::{KeyCode, KeyEvent};
use cyclic_list::{CyclicList, List};
use log::trace;

use crate::model::Model;

use super::{Game, score::Score, CursorCommand, Board, CursorMove, CursorRot, CursorLen, cursor::Coord, cell::Cell};

const WIDTH : usize = 20;
const HEIGHT : usize = 34;

pub type GameState = (Board<WIDTH, HEIGHT>, (Coord, Coord), u64);

#[derive(Debug)]
pub enum GameCommand{
    BoardState(GameState),
    GameOver(Score)
}

type UserCommandQueue = CyclicList<20, CursorCommand, false>;

pub struct GameModel<const GAME_TICK_RATE: u64>{
    _thread: JoinHandle<()>,
    game: Game::<WIDTH, HEIGHT>,
    update_cond: Receiver<()>,
    total_ticks: u128,
    update: bool,
}

impl<const GAME_TICK_RATE: u64> GameModel<GAME_TICK_RATE> {
    pub fn new(max_tick: u64) -> Self {
        let mut game = Game::<WIDTH, HEIGHT>::default();
        let (tx, rx) = mpsc::channel();

        let update_cond = rx;

        let _thread = thread::spawn(move || {
            loop {
                tx.send(());
                thread::sleep(Duration::from_millis(250));
            }
        });

        return Self{
            _thread,
            game,
            update_cond,
            total_ticks: 0,
            update: true
        }
    }
}


impl<const T: u64> Model<KeyEvent, Option<GameCommand>> for GameModel<T> {
    fn update_self(&mut self, event: KeyEvent) {

        self.update = true;
        match event.code {
            KeyCode::Left | KeyCode::Char('A') | KeyCode::Char('a') | KeyCode::Right | KeyCode::Char('D') | KeyCode::Char('d') | KeyCode::Up | KeyCode::Char('W') | KeyCode::Char('w') | KeyCode::Down | KeyCode::Char('S') | KeyCode::Char('s') | KeyCode::Char('q') | KeyCode::Char('e') | KeyCode::Char('Q') | KeyCode::Char('E') | KeyCode::Char('X') | KeyCode::Char('V') => {
                let command = game_command_update(event);

                match command {
                    CursorCommand::Move(command) => {
                        match command {
                            super::CursorMove::X(x) => self.game.move_cursor(x.clone(), 0),
                            super::CursorMove::Y(y) => self.game.move_cursor(0, y.clone()),
                        }
                    },
                    CursorCommand::Len(command) => self.game.change_cursor_length(command.clone()),
                    CursorCommand::Rotate(command) => self.game.change_cursor_rot(command.clone()),
                    CursorCommand::Cut => self.game.cut_at_cursor(),
                    CursorCommand::Paste => self.game.paste_at_cursor(),
                    CursorCommand::Nothing => {},
                }
            },

            KeyCode::Esc => panic!("pre mature end"),

            _ => {

            }
        }
        

        if self.total_ticks % min(20, 1000 / max(self.total_ticks, 1)) == 0 {
            self.game.next();
        }

    
        self.total_ticks+=1;
        
        
        
    }

    fn update_presenter(&mut self) -> Option<GameCommand> {
        if self.update {
            self.update = false;
            return Some(GameCommand::BoardState((
                self.game.board,
                self.game.cursor_range(),
                0
            )));
        }

        let end_game = self.game.board
            .iter()
            .all(
                |row| {
                    row.iter()
                    .any(|cell| {
                        if let Cell::Empty = cell {
                            return false;
                        }
                        return true;
                    })
                }
            );

        if end_game {
            return Some(GameCommand::GameOver(self.game.clone().into()))
        }

        if self._thread.is_finished() {
            trace!("It's over?");
        }
        None
    }
}

fn game_command_update(event: KeyEvent) -> CursorCommand {
    match event.code {
        KeyCode::Left | KeyCode::Char('A') | KeyCode::Char('a') => {
            return CursorCommand::Move(CursorMove::X(-1));
        },
        KeyCode::Right | KeyCode::Char('D') | KeyCode::Char('d') => {
            return CursorCommand::Move(CursorMove::X(1));
        },
        KeyCode::Up | KeyCode::Char('W') | KeyCode::Char('w') => {
            return CursorCommand::Move(CursorMove::Y(1));
        },
        KeyCode::Down | KeyCode::Char('S') | KeyCode::Char('s') => {
            return CursorCommand::Move(CursorMove::Y(-1));
        },

        KeyCode::Char('q') => {
            return CursorCommand::Rotate(CursorRot::CounterClockwise);
        },
        KeyCode::Char('e') => {
            return CursorCommand::Rotate(CursorRot::Clockwise);
        },

    
        KeyCode::Char('Q') => {
            return CursorCommand::Len(CursorLen::Shrink);
        },
        KeyCode::Char('E') => {
            return CursorCommand::Len(CursorLen::Grow);
        },

        KeyCode::Char('X') => {
            return CursorCommand::Cut;
        },
        KeyCode::Char('V') => {
            return CursorCommand::Paste;
        },

        _ => panic!("{:?}\n WOAH HOW DID THAT GO THROUGH", event)
    }
}