use crossterm::event::KeyEvent;
use cyclic_list::{CyclicList, List};
use log::trace;


use crate::{model::{cut_and_paste::SceneModel, Model, ui::cut_scene::{CutSceneState, CutSceneModel}, game::game_model::GameCommand}, view::{terminal::{TerminalView, TerminalUpdate}, io::input_handler::InputQueue, View}};

use super::{menu::{MainMenu, MenuCommandQueue}, cut_scene::CutScene, Presenter, game_scene::GameScene};

pub enum ScenePresenter{
    MainMenu(MainMenu),
    CutScene(CutScene),
    Game(GameScene),
    GameOver(CutScene)
}

impl Default for ScenePresenter{
    fn default() -> Self {
        Self::MainMenu(MainMenu::new())
    }
}

impl TryFrom<u32> for ScenePresenter {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ScenePresenter::MainMenu(MainMenu::new())),
            1 => Ok(ScenePresenter::CutScene(CutScene::new())),
            2 => Ok(ScenePresenter::Game(GameScene::new())),
            3 => Ok(ScenePresenter::CutScene(CutScene::new())),
            _ => Err(())
        }
    }
}

pub enum PresenterCommand {
    Change(u32),

    MainMenu(MenuCommandQueue),
    CutScene(CutSceneState),
    GameScene(Option<GameCommand>),
}


impl<const T: u64> Presenter<SceneModel, TerminalView<T>, KeyEvent, InputQueue, Box<TerminalUpdate>, Option<PresenterCommand>> for ScenePresenter {
    fn update_model(&self, model: &mut SceneModel, view: &mut TerminalView<T>) {

        let events = view.send_event();
        let events = &mut *events.lock().unwrap();

        while let Some(event) = events.remove_front() {
            let event = event.clone().unwrap();
            model.update_self(event);
        }
    }

    fn update_view(&mut self, model: &mut SceneModel, view: &mut TerminalView<T>, _cmd_carry_over: Option<Option<PresenterCommand>>) {
        let commands = model.update_presenter();

        if let Some(command) = commands {
            match command {
                PresenterCommand::Change(i) => {
                    match (Self::try_from(i), SceneModel::try_from(i)) {
                        (Ok(new_presenter), Ok(new_model)) => {
                            *self = new_presenter;
                            *model = new_model;

                        },
                        _ => {
                            
                        }
                    }
                },
                PresenterCommand::MainMenu(_cmd) => {
                    if let (ScenePresenter::MainMenu(presenter), SceneModel::MainMenu(model)) = (self, model) {
                        presenter.update_view(model, view, None);
                    }
                },
                PresenterCommand::CutScene(cut_scene) => {
                    match (self, model) {
                        (ScenePresenter::CutScene(presenter), SceneModel::CutScene(model)) => presenter.update_view(model, view, Some(Some(cut_scene))),
                        (ScenePresenter::CutScene(presenter), SceneModel::GameOverScene(model)) => presenter.update_view(model, view, Some(Some(cut_scene))),
                        (ScenePresenter::GameOver(presenter), SceneModel::CutScene(model)) => presenter.update_view(model, view, Some(Some(cut_scene))),
                        (ScenePresenter::GameOver(presenter), SceneModel::GameOverScene(model)) => presenter.update_view(model, view, Some(Some(cut_scene))),
                        _ => {}
                    }
                },
                PresenterCommand::GameScene(game) => {
                    match game {
                        Some(GameCommand::GameOver(score)) => {
                            trace!("GAME OVER change scene");
                            *self = Self::try_from(1).unwrap();
                            *model = SceneModel::GameOverScene(
                                CutSceneModel::new(
                                    vec![
                                        format!("Completed rows: {}", score.rows),
                                        format!("Completed Columns: {}", score.cols),
                                        format!("Score: {}", score.score()),
                                    ]
                                )
                            );

                            return ;
                        },
                        Some(GameCommand::BoardState(board)) => {
                            if let (ScenePresenter::Game(game_presenter), SceneModel::GameScene(game_model)) = (self, model) {
                                game_presenter.update_view(game_model, view, Some(game))
                            }
                        },
                        None => {
                        },
                    };
                    
                },
            }
        }
    }
}