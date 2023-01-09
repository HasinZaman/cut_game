use std::io::Stdout;

use crossterm::event::KeyEvent;
use cyclic_list::{List, CyclicList};
use log::trace;
use tui::{layout::{Direction, Constraint, Layout, Alignment}, widgets::{Paragraph}, text::{Spans, Span}, style::{Style, Color}, Frame, backend::CrosstermBackend};

use crate::{model::{Model, ui::{menu::{MainMenuOption, MenuCommand}}}, view::{View, terminal::{TerminalView, TerminalUpdate}, io::input_handler::InputQueue}};

use super::Presenter;

use crate::model::ui::menu::Menu;

pub struct MainMenu;

impl MainMenu {
    pub fn new() -> Self {
        Self
    }
}

pub type MenuCommandQueue = CyclicList<5, MenuCommand, true>;

impl<const D: u64> Presenter<Menu<MainMenuOption>, TerminalView<D>, KeyEvent, InputQueue, Box<TerminalUpdate>, MenuCommandQueue> for MainMenu {
    fn update_model(&self, model: &mut Menu<MainMenuOption>, view: &mut TerminalView<D>) {
        let events = view.send_event();
        let events = &mut *events.lock().unwrap();

        while let Some(event) = events.remove_front() {
            let event = event.clone().unwrap();
            model.update_self(event);
        }
    }

    fn update_view(&mut self, model: &mut Menu<MainMenuOption>, view: &mut TerminalView<D>, cmd_carry_over: Option<MenuCommandQueue>) {

        let mut commands = match cmd_carry_over {
            Some(val) => val,
            None => model.update_presenter(),
        };

        while let Some(command) = commands.remove_front() {
            match command {
                MenuCommand::Select(_cmd) => {
                    panic!("Command should have been brought up a level")
                },
                MenuCommand::UpdateView => {
                    view.update(render_fn(model));
                },
            }
        }

        
    }

}

fn render_fn(model: &mut Menu<MainMenuOption>) -> Box<dyn Fn(&mut Frame<CrosstermBackend<Stdout>>)> {
    let title = model.title.clone();
    let menu_options = Menu::<MainMenuOption>::options();
    let selected = model.menu as usize;
    let render_fn: Box<TerminalUpdate> = Box::new(
        move |f| {

            let lines: Vec<&str> = title.split("\n").collect();

            let white_space = (f.size().height - (lines.len() as u16 + 2) - (menu_options.len() as u16)) / 3;

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(white_space),
                        Constraint::Length(lines.len() as u16 + 2),
                        Constraint::Length(white_space),
                        Constraint::Length(menu_options.len() as u16),
                        Constraint::Length(white_space),
                    ]
                )
                .split(f.size());
                
            if lines.clone().len() > 0 {
                let chunk = chunks[1];
            
                let width = lines.iter()
                    .max()
                    .unwrap()
                    .len() as u16 + 2;

                let white_space = (chunk.width - (width)) / 2;

                let title_rect = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(
                        [
                            Constraint::Length(white_space),
                            Constraint::Length(width),
                            Constraint::Length(white_space),
                        ]
                    )
                    .split(chunk);
            

                f.render_widget(
                    Paragraph::new(
                        lines.iter()
                            .map(|line| Spans::from(vec![Span::from(*line)]))
                            .collect::<Vec<Spans>>()
                    )
                    .alignment(Alignment::Center),
                    title_rect[1]
                );
            }
    
            if menu_options.len() > 0 {
                let chunk = chunks[3];
            
                let width = menu_options.iter()
                    .max()
                    .unwrap()
                    .len() as u16;

                let white_space = (chunk.width - (width)) / 2;

                let menu_rect = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(
                        [
                            Constraint::Length(white_space),
                            Constraint::Length(width),
                            Constraint::Length(white_space),
                        ]
                    )
                    .split(chunks[3]);
            
                f.render_widget(
                    Paragraph::new(
                        menu_options.iter()
                            .enumerate()
                            .map(
                                |(index, option)| {
                                    Spans::from(vec![
                                        menu_option_span(option.to_string(), selected == index)
                                    ])
                                }   
                            ).collect::<Vec<Spans>>()    
                    )
                    ,menu_rect[1]
                )
            }
        }
    );
    render_fn
}

fn menu_option_span<'a>(text: String, selected: bool) -> Span<'a> {
    let (style, text) = match selected {
        false => (
            Style::default()
                .bg(Color::Black)
                .fg(Color::White),
            
            (0..text.len())
                .map(|_| random_char())
            .collect::<String>()
        ),
        true => (
            Style::default()
                .bg(Color::White)
                .fg(Color::Black),
            text
        )
    };

    Span::styled(
        text,
        style
    )
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