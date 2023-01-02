use std::{time::Duration, thread};

use game::row::Row;

mod game;
fn main() {
    //let mut game = game::Game::<20,32>::default();

    let mut game = game::Game::<4,8>::default();


    // for i in 30..32{
    //     row_mut::random(&mut game.board[i]);
    // }

    for i in 6..8{
        Row::random(&mut game.board[i]);
    }


    Row::random(&mut game.board[0]);
    loop {
        println!("{:?}", game);
        game.next();
        
        thread::sleep(Duration::from_secs(1));
    }
    

}
