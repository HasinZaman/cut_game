use crate::{view::View, model::{Model}};

pub mod menu;
pub mod cut_scene;
pub mod game_scene;
pub mod cut_and_paste;


pub trait Presenter<M: Model<E, P>, V: View<T, D>, E, T, D, P>{
    fn update_model(&self, model: &mut M, view: &mut V);
    fn update_view(&mut self, model: &mut M, view: &mut V, cmd_carry_over: Option<P>);
}
