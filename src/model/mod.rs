pub mod game;
pub mod ui;
pub mod cut_and_paste;

pub trait Model<Event, PresenterCommand>{
    fn update_self(&mut self, event: Event);
    fn update_presenter(&mut self) -> PresenterCommand;
}