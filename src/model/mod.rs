pub mod game;
pub mod ui;

pub trait Model<E>{
    fn update(&mut self, event: E);
}