use std::{thread::{self, JoinHandle}, time::Duration, sync::mpsc::{self, Sender, Receiver}};

use crossterm::event::{KeyEvent, KeyCode};


use crate::model::Model;

#[derive(Debug, Clone)]
pub enum CutSceneState {
    UpdateMessage(String),
    Completed,
}

pub struct CutSceneModel<const UPDATE_RATE: u64> {
    thread: JoinHandle<()>,
    change_scene: bool,
    rx: Receiver<String>,
    tx: Sender<bool>,
}

impl<const UPDATE_RATE: u64> CutSceneModel<UPDATE_RATE> {
    pub fn new(messages: Vec<String>) -> Self {

        let (cut_scene_tx, rx) = mpsc::channel();
        let (tx, cut_scene_rx) = mpsc::channel();

        let thread = thread::spawn(move || {
            let mut i1 = 0;

            let messages = messages;

            let mut messages = messages.iter();
            let mut message = messages.next();

            let rx = rx;
            let tx = tx;

            let _ = tx.send(String::from(""));
            loop {
                if let Ok(true) = rx.try_recv() {
                    if i1 <= message.unwrap().len() {
                        i1 = message.unwrap().len();
                    }
                    else {
                        message = messages.next();
                        i1 = 0;
                    }
                }

                if let None = message {
                    break;
                }


                //update cut message
                if let Some(message) = message.unwrap().get(0..i1) {
                    let _ = tx.send(message.to_string());
                }

                thread::sleep(Duration::from_millis(UPDATE_RATE));

                i1 += 1;
            }
        });

        let rx = cut_scene_rx;
        let tx = cut_scene_tx;

        Self{
            thread,
            rx,
            tx,
            change_scene: false
        }
    }
}

impl<const U: u64> Model<KeyEvent, Option<CutSceneState>> for CutSceneModel<U> {
    fn update_self(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Enter => {
                match self.thread.is_finished() {
                    false => {
                        let _ = self.tx.send(true);
                    },
                    true => {
                        self.change_scene = true;
                    },
                };
            },
            _ => {},
        }
    }

    fn update_presenter(&mut self) -> Option<CutSceneState> {
        if self.thread.is_finished() && self.change_scene{
            return Some(CutSceneState::Completed)
        }

        self.rx.try_recv()
            .ok()
            .map(|val| {
                CutSceneState::UpdateMessage(val)
            })
    }
}