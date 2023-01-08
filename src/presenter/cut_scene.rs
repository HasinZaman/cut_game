use std::{io::Stdout, cmp::{min, max}};

use crossterm::event::KeyEvent;
use cyclic_list::List;
use tui::{Frame, backend::CrosstermBackend, layout::{Constraint, Layout, Direction, Alignment, Rect}, widgets::Paragraph, text::{Spans, Span}, style::{Style, Color}};

use crate::{model::{ui::cut_scene::{CutSceneModel, CutSceneState}, Model}, view::{terminal::{TerminalView, TerminalUpdate}, io::input_handler::InputQueue, View}};

use super::Presenter;


pub struct CutScene;

impl CutScene {
    pub fn new() -> Self {
        Self
    }
}

impl<const D1: u64, const D2: u64> Presenter<CutSceneModel<D2>, TerminalView<D1>, KeyEvent, InputQueue, Box<TerminalUpdate>, Option<CutSceneState>> for CutScene{
    fn update_model(&self, model: &mut CutSceneModel<D2>, view: &mut TerminalView<D1>) {
        let events = view.send_event();
        let events = &mut *events.lock().unwrap();

        while let Some(event) = events.remove_front() {
            let event = event.clone().unwrap();
            model.update_self(event);
        }
    }

    fn update_view(&mut self, model: &mut CutSceneModel<D2>, view: &mut TerminalView<D1>, cmd_carry_over: Option<Option<CutSceneState>>) {

        let model_cmd = match cmd_carry_over {
            Some(val) => val,
            None => model.update_presenter(),
        };

        if let None = model_cmd {
            return ;
        }

        let model_cmd = model_cmd.unwrap();

        match model_cmd {
            CutSceneState::UpdateMessage(message) => {

                view.update(render_fn(message));
            },
            CutSceneState::Completed => panic!("Should have gone to game scene"),
        }
    }
}

fn render_fn(message: String) -> Box<dyn Fn(&mut Frame<CrosstermBackend<Stdout>>)> {
    
    let render_fn: Box<TerminalUpdate> = Box::new(
        move |f| {

            //max margin of n Text n

            let height = f.size().height;
            let width = f.size().width;

            let lines = max(message.len() as u16 / width * 8 / 10, 2);

            let white_space = (height - lines - 1) / 2;

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(white_space),
                        Constraint::Length(lines),
                        Constraint::Length(white_space),
                    ]
                )
                .split(f.size());
                
            if lines > 0 {

                let chunk = chunks[1];

                let width = width * 8 / 10;

                let white_space = (chunk.width - (width)) / 2;

                let message_rect = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(
                        [
                            Constraint::Length(white_space),
                            Constraint::Length(width),
                            Constraint::Length(white_space),
                        ]
                    )
                    .split(chunk)[1];
            

                f.render_widget(
                    Paragraph::new(
                        {
                            let mut spans = vec![Spans::from("")];

                            for i in 0..lines as usize {

                                let start = i * width as usize;
                                let end = min((i + 1) * width as usize, message.len());

                                if start > message.len() {
                                    break;
                                }

                                let line = message.get(start..end).unwrap();

                                match i {
                                    0 => {
                                        spans[0] = Spans::from(line);
                                    },
                                    _ => {
                                        spans.push(
                                            Spans::from(line)
                                        )
                                    },
                                };
                            }

                            spans.last_mut()
                                .unwrap()
                                .0
                                .push(
                                    Span::styled(
                                        format!("{}", random_char()),
                                        Style::default()
                                    )
                                );

                            spans
                        }
                    )
                    .alignment(Alignment::Center),
                    message_rect
                );
            }
    
            {
                let next = "Press ENTER to Skip";

                let chunk = Rect{
                    x: width - 1 - next.len() as u16,
                    y: height - 1,
                    width: next.len() as u16,
                    height: 1,
                };

                f.render_widget(
                    Paragraph::new(
                        {
                            
                            vec![
                                Spans::from(
                                    vec![
                                        Span::from("Press "),

                                        Span::styled(
                                            "ENTER",
                                            Style::default()
                                                .bg(Color::White)
                                                .fg(Color::Black)
                                        ),
                                        Span::from(" to Skip"),
                                    ]
                                    
                                )
                            ]
                        }
                    ),
                    chunk
                );
            }
        }
    );
    render_fn
}


fn random_char() -> char{
    const RANGE : u8 = (57-48) + (90-65);// + (122-97);
    let rand_char = rand::random::<u8>() % RANGE;

    let rand_char = match rand_char {
        0..=9 => rand_char + 48,
        10..=35 => rand_char + 65,
        36.. => {
            rand_char+97
        }
    };

    rand_char as char
    // 48-57 # num
    // 65-90 # cap letter
    // 97-122 # lower letter
}