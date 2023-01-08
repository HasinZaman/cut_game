use logger::log_initialize;

use model::{cut_and_paste::SceneModel};
use presenter::{Presenter, cut_and_paste::ScenePresenter};



use view::{terminal::{TerminalView}};

mod logger;

mod model;
mod presenter;
mod view;

fn main() {

    log_initialize();    


    let mut model = SceneModel::default();
    let mut presenter = ScenePresenter::default();//MainMenu::new();
    let mut terminal = TerminalView::<16>::new(Box::new(|_| {}));

    loop {
        presenter.update_model(&mut model, &mut terminal);
        
        presenter.update_view(&mut model, &mut terminal, None);
        
        terminal.render();
    }

    // let mut game = game::Game::<5,5>::default();


    // Row::random(&mut game.board[0]);

    // let (_t, io_stream) = input_handler();
    
    // loop {
    //     {
    //         let mut io_stream = io_stream.lock().unwrap();
    //         let io_stream = &mut *io_stream;

    //         while io_stream.len() > 0 {
    //             let event = io_stream.remove_front().unwrap();

    //             match event {
    //                 io::GameEvent::Key(char) => {
    //                     match char {
    //                         //move cursor
    //                         'w' => {
    //                             game.move_cursor(0, 1);
    //                         },
    //                         'a' => {
    //                             game.move_cursor(-1, 0);
    //                         },
    //                         's' => {
    //                             game.move_cursor(0, -1);
    //                         },
    //                         'd' => {
    //                             game.move_cursor(1, 0);
    //                         },

    //                         //rotate
    //                         'q' => {
    //                             game.change_cursor_rot(CursorRotChangeDir::Clockwise);
    //                         },
    //                         'e' => {
    //                             game.change_cursor_rot(CursorRotChangeDir::CounterClockwise);
    //                         },

    //                         //rotate
    //                         'Q' => {
    //                             game.change_cursor_length(CursorPosChangeDir::Shrink);
    //                         },
    //                         'E' => {
    //                             game.change_cursor_length(CursorPosChangeDir::Grow);
    //                         },

    //                         //cut and paste
    //                         'x' => {
    //                             game.cut_at_cursor();
    //                         },
    //                         'v' => {
    //                             game.paste_at_cursor();
    //                         },

    //                         _ => {

    //                         }
    //                     }
    //                 },
    //             }
    //         }
    //     }
        
    //     println!("{:?}", game);
    //     game.calc_point();
    //     game.next();

    //     thread::sleep(Duration::from_millis(1000));
    // }
    

}

