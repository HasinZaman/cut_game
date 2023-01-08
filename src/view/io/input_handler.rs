use std::{thread::{JoinHandle, self}, sync::{Arc, Mutex, mpsc::{channel, Receiver}}};

use crossterm::event::{self, KeyEvent};
use cyclic_list::{CyclicList, list::List};



pub type InputQueue = CyclicList<10, Option<KeyEvent>, false>;

pub struct InputHandler{
    _thread: JoinHandle<()>,

    pub view_queue: Arc<Mutex<InputQueue>>,
    pub focus: Receiver<bool>,
    pub new_size: Receiver<(u16,u16)>,
}

impl InputHandler {
    pub fn new() -> Self {
        let (
            thread,
            view_queue,
            focus,
            new_size
        ) = input_handler();

        Self{
            _thread: thread,

            view_queue,
            focus,
            new_size
        }
    }
}


fn input_handler() -> (JoinHandle<()>, Arc<Mutex<InputQueue>>, Receiver<bool>, Receiver<(u16,u16)>) {
    let input_stream = Arc::new(
        Mutex::new(
            InputQueue::default()
        )
    );

    let (focus_tx, focus_rx) = channel();
    
    let (new_size_tx, new_size_rx) = channel();

    let t_input_stream = input_stream.clone();

    let t = thread::spawn(move || {

        loop {
            let event = event::read();

            if let Err(_err) = event {
                continue;
            }

            let event = event.unwrap();

            match event {
                //presenter related event
                event::Event::FocusGained => {
                    let _ = focus_tx.send(true);
                },
                event::Event::FocusLost => {
                    let _ = focus_tx.send(false);
                },

                //view related event
                event::Event::Resize(col, row) => {
                    let _ = new_size_tx.send((col, row));
                },

                //model 
                event::Event::Key(key) => key_event(key, &mut *t_input_stream.lock().unwrap()),
                event::Event::Mouse(_) | event::Event::Paste(_) => {}
                
            }
        }
    });

    (t, input_stream, focus_rx, new_size_rx)
}


fn key_event(key: KeyEvent, io_stream: &mut InputQueue) {
    let _ = io_stream.push(Some(key));
}