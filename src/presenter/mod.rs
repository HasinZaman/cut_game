use crate::{view::View, model::Model};

pub mod menu;



pub trait Presenter<M: Model<E>, V: View<T, D>, E, T, D>{
    fn update_model(&self, model: &mut M, view: &mut V);
    fn update_view(&self, model: &mut M, view: &mut V);
}
