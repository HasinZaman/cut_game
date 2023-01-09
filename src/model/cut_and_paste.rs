use std::{fs, path::Path};

use crossterm::event::KeyEvent;
use cyclic_list::List;
use log::trace;

use crate::presenter::{cut_and_paste::{PresenterCommand}};

use super::{ui::{menu::{Menu, MainMenuOption, MenuCommand}, cut_scene::{CutSceneModel, CutSceneState}}, Model, game::game_model::GameModel};



pub enum SceneModel{
    MainMenu(Menu<MainMenuOption>),
    CutScene(CutSceneModel<100>),
    GameScene(GameModel),
    GameOverScene(CutSceneModel<100>),
}

impl TryFrom<u32> for SceneModel {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SceneModel::MainMenu(
                Menu::<MainMenuOption>::new(
                    fs::read_to_string(Path::new("title.txt")).unwrap(),
                    MainMenuOption::default()
                )
            )),
            1 => Ok(SceneModel::CutScene(CutSceneModel::new(
                vec![
                    String::from("Fill rows or columns with only '1's or '0' to get points"),
                    String::from("use 'W', 'A', 'S', 'D' to move"),
                    String::from("use 'Q' & 'E' to expand and shrink the cursor"),
                    String::from("use 'q' & 'e' to rotate the cursor"),
                    String::from("use 'X' & 'V' to cut and paste"),
                    ]
                ))),
            2 => Ok(
                SceneModel::GameScene(
                    GameModel::new(1000)
                )
            ),
            3 => Ok(SceneModel::CutScene(CutSceneModel::new(
                vec![
                    String::from("Your Score was "),
                    String::from("use 'W', 'A', 'S', 'D' to move"),
                    String::from("use 'Q' & 'E' to expand and shrink the cursor"),
                    String::from("use 'q' & 'e' to rotate the cursor"),
                    String::from("use 'X' & 'V' to cut and paste"),
                    ]
                ))),
            _ => Err(())
        }
    }
}

impl Default for SceneModel {
    fn default() -> Self {
        Self::MainMenu(
            Menu::<MainMenuOption>::new(
                fs::read_to_string(Path::new("title.txt")).unwrap(),
                MainMenuOption::default()
            )
        )
    }
}


impl Model<KeyEvent, Option<PresenterCommand>> for SceneModel {
    fn update_self(&mut self, event: KeyEvent) {
        match self {
            SceneModel::MainMenu(main_menu) => main_menu.update_self(event),
            SceneModel::CutScene(cut_scene) => cut_scene.update_self(event),
            SceneModel::GameScene(game_scene) => game_scene.update_self(event),
            SceneModel::GameOverScene(game_over_scene) => game_over_scene.update_self(event),
        }
    }

    fn update_presenter(&mut self) -> Option<PresenterCommand> {
        match self{
            SceneModel::MainMenu(main_menu) => {
                let menu_commands = main_menu.update_presenter();

                if menu_commands.len() == 0 {
                    return None;
                }

                let change_scene = menu_commands.clone()
                    .iter()
                    .any(|command| {
                        if let MenuCommand::Select(scene_name) = command {
                            let scene_name = &**scene_name;
                            match scene_name {
                                "Play" => {return true},
                                _ => {return true},
                            }
                        }
                        let _ = main_menu.menu_commands.push(command.clone());
                        false
                    });
                if change_scene {
                    trace!("Change scene");
                    return Some(PresenterCommand::Change(1));
                }
                else {
                    trace!("Regular menu commands");
                    return Some(PresenterCommand::MainMenu(menu_commands));
                }
            },
            SceneModel::GameOverScene(cut_scene) | SceneModel::CutScene(cut_scene) => {
                let command = cut_scene.update_presenter();

                if let None = command {
                    return None;
                }

                let command = command.unwrap();

                match command {
                    CutSceneState::UpdateMessage(message) => {
                        return Some(
                            PresenterCommand::CutScene(
                                CutSceneState::UpdateMessage(
                                    message
                                )
                            )
                        )
                    },
                    CutSceneState::Completed => {
                        return Some(PresenterCommand::Change(2));
                    },
                }
            },
            SceneModel::GameScene(game_scene) => {
                let command = game_scene.update_presenter();

                return Some(PresenterCommand::GameScene(command))
            },
        }
    }
}
