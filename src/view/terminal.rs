
use std::{io::{self, Stdout}, thread::{self, JoinHandle}, time::Duration, sync::{Mutex, Arc}};

use crossterm::{execute, terminal::EnterAlternateScreen};
use tui::{Terminal, backend::{CrosstermBackend}, Frame, layout::Rect};

use super::{io::input_handler::{InputHandler, InputQueue}, View};

pub type TerminalUpdate = dyn Fn(&mut Frame<CrosstermBackend<Stdout>>);

pub struct TerminalView<const FRAME_DELTA_TIME: u64>{
    input_handler: InputHandler,
    
    backend: Terminal<CrosstermBackend<Stdout>>,

    update: Box<TerminalUpdate>,
    focus: bool,

    _t: Option<JoinHandle<()>>
}

impl<const FRAME_DELTA_TIME: u64> View<InputQueue, Box<TerminalUpdate>> for TerminalView<FRAME_DELTA_TIME> {
    fn send_event(&self) -> Arc<Mutex<InputQueue>> {
        self.input_handler.view_queue.clone()
    }

    fn update(&mut self, data: Box<TerminalUpdate>) {
        let _ = self.backend.clear();
        self.update = data;
    }
}

impl<const FRAME_DELTA_TIME: u64> TerminalView<FRAME_DELTA_TIME>{
    pub fn new(update: Box<TerminalUpdate>) -> Self {
        let stdout = io::stdout();
        
        let _result = execute!(io::stdout(), EnterAlternateScreen);

        let backend = CrosstermBackend::new(stdout);

        let mut terminal = match Terminal::new(backend){
            Ok(val) => val,
            Err(err) => panic!("{}", err)
        };

        let _ = terminal.hide_cursor();

        Self {
            input_handler: InputHandler::new(),
            backend: terminal,

            update,
            focus: true,

            _t: None
        }
    }

    pub fn render(&mut self) {
        if let Ok(focus) = self.input_handler.focus.try_recv() {
            self.focus = focus;
        }

        if let Ok(size) = self.input_handler.new_size.try_recv() {
            let _ = self.backend.resize(Rect{
                x: 0,
                y: 0,
                width: size.0,
                height: size.1,
            });
        }

        if self.focus {
            //self.backend.clear();
            let _ = self.backend.draw(|f| {
                (*self.update)(f);
            });
        }

        thread::sleep(Duration::from_millis(FRAME_DELTA_TIME));
    }

}