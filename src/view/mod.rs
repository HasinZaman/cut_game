use std::sync::{Mutex, Arc};

pub(crate) mod io;
pub mod terminal;

pub trait View<T, D>{
    fn send_event(&self) -> Arc<Mutex<T>>;
    fn update(&mut self, data: D);
}