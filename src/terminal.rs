
use std::io;

use tui::{Terminal, backend::{CrosstermBackend}};


pub fn terminal_initialize() -> Terminal<CrosstermBackend<std::io::Stdout>> {
    let mut stdout = io::stdout();

    // let _result = execute!(stdout, EnterAlternateScreen);

    let backend = CrosstermBackend::new(stdout);
    match Terminal::new(backend){
        Ok(val) => val,
        Err(err) => panic!("{}", err)
    }
}