use std::{io::Stdout, cmp::{min, max}};

use crossterm::event::{KeyEvent, KeyCode, KeyModifiers, KeyEventKind, KeyEventState};
use cyclic_list::List;

use tui::{Frame, backend::CrosstermBackend, layout::{Layout, Direction, Constraint, Rect, Alignment}, text::{Span, Spans}, style::{Style, Color}, widgets::{Paragraph, Gauge, Block, Borders}};

use crate::{model::{game::{game_model::{GameModel, GameCommand}, Board, cell::{Cell, CellValue}}, Model}, view::{terminal::{TerminalView, TerminalUpdate}, io::input_handler::InputQueue, View}};

use super::Presenter;


pub struct GameScene;

impl GameScene {
    pub fn new() -> Self {
        Self
    }
}

impl<const D: u64> Presenter<GameModel, TerminalView<D>, KeyEvent, InputQueue, Box<TerminalUpdate>, Option<GameCommand>>  for GameScene {
    fn update_model(&self, model: &mut GameModel, view: &mut TerminalView<D>) {
        let events = view.send_event();
        let events = &mut *events.lock().unwrap();

        while let Some(event) = events.remove_front() {
            let event = event.clone().unwrap();
            model.update_self(event);
        }
        model.update_self(KeyEvent { code: KeyCode::Enter, modifiers: KeyModifiers::NONE, kind: KeyEventKind::Press, state: KeyEventState::NONE });
    }

    fn update_view(&mut self, model: &mut GameModel, view: &mut TerminalView<D>, cmd_carry_over: Option<Option<GameCommand>>) {
        let command = match (model.update_presenter(), cmd_carry_over) {
            (Some(_), None) | (None, None) => None,
            (Some(_), Some(val)) | (None, Some(val)) => val,
        };

        if let None = command {
            return;
        }
        let command = command.unwrap();

        match command {
            GameCommand::BoardState((board, cursor_range, end_state)) => {
                view.update(render_fn(board, cursor_range, end_state));
            },
            GameCommand::GameOver(score) => {
                let mut score = score.clone();
                score.col_modifier = 100;
                score.row_modifier = 10;
                panic!("YOUR SCORE IS {}", score.score());
                
            },
        }

    }
}

fn render_fn<const WIDTH: usize, const HEIGHT: usize>(board: Board<WIDTH, HEIGHT>, cursor_range: ((i8, i8), (i8, i8)), end_state: u64) -> Box<dyn Fn(&mut Frame<CrosstermBackend<Stdout>>)> {
    let render_fn: Box<TerminalUpdate> = Box::new(
        move |f| {
            {
                let size = f.size();

                if size.width < WIDTH as u16 || size.height < HEIGHT as u16 {

                    f.render_widget(
                        Paragraph::new(
                            Spans::from(
                                vec![
                                    Span::from("Game Screen is too small. Resize window to fit game screen.")
                                ]
                            )
                        )
                        .alignment(Alignment::Center),
                        f.size()
                    );
                    return;
                }
            }

            let white_space = (f.size().height - HEIGHT as u16)/ 2;

            let mut board_rect = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(white_space),
                        Constraint::Length(HEIGHT as u16 + 2),
                        Constraint::Length(white_space),
                    ]
                )
                .split(f.size())[1];

            //board
            {
                let white_space = (f.size().width - WIDTH as u16)/ 2;

                board_rect = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Length(white_space),
                        Constraint::Length(WIDTH as u16 + 2),
                        Constraint::Length(white_space),
                    ]
                )
                .split(board_rect)[1];

                let x_range = (min(cursor_range.0.0, cursor_range.1.0), max(cursor_range.0.0, cursor_range.1.0));
                let y_range = (min(cursor_range.0.1, cursor_range.1.1), max(cursor_range.0.1, cursor_range.1.1));

                let mut lines = vec![];
                for row in (0..HEIGHT).rev() {
                    let mut spans = vec![];
                    for col in 0..WIDTH {
                        let x_cond = x_range.0 <= col as i8 && col as i8 <= x_range.1;
                        let y_cond = y_range.0 <= row as i8 && row as i8 <= y_range.1;

                        let cell = match board[row][col] {
                            Cell::Empty | Cell::Filled(_) => board[row][col].to_string(),
                            Cell::Random => format!("{}", rand::random::<CellValue>() as u8),
                        };
                        match (x_cond, y_cond) {
                            (true, true) => {
                                
                                spans.push(
                                    Span::styled(
                                        cell,
                                        Style::default()
                                            .bg(Color::White)
                                            .fg(Color::Black)
                                    )
                                );
                            },
                            _ => {
                                spans.push(
                                    Span::from(cell)
                                );
                            }
                        }
                    }

                    lines.push(Spans::from(spans));
                }

                f.render_widget(
                    Paragraph::new(lines)
                        .block(Block::default().borders(Borders::all())),
                    board_rect
                );
            }

            //
            {
                let end_game_bar = Rect{
                    x: board_rect.x+board_rect.width+1,
                    y: board_rect.y,
                    width: 1,
                    height: board_rect.height,
                };// board_rect;

                f.render_widget(
                    Gauge::default()
                        .gauge_style(Style::default())
                        .percent(end_state as u16),
                    end_game_bar
                )
            }
        }
    );
    render_fn
}


// fn random_char() -> char{
//     const RANGE : u8 = (57-48) + (90-65);// + (122-97);
//     let rand_char = rand::random::<u8>() % RANGE;

//     let rand_char = match rand_char {
//         0..=9 => rand_char + 48,
//         10..=35 => rand_char + 65,
//         36.. => {
//             rand_char+97
//         }
//     };

//     rand_char as char
//     // 48-57 # num
//     // 65-90 # cap letter
//     // 97-122 # lower letter
// }