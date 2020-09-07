pub mod end_scene;
pub mod fight_scene;
pub mod io_interactions;
pub mod text_scene;

/// Converter from string to SceneType
impl SceneType {
    fn from_string(state: &str) -> Self {
        match state {
            "normal" => SceneType::Normal,
            "fight" => SceneType::Fight,
            "end" => SceneType::End,
            _ => SceneType::End,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum SceneType {
    Normal,
    Fight,
    End,
}

pub trait Scene {
    fn playout(&self) -> String;
    fn get_scene_text(&self) -> String;
    fn get_scene_type(&self) -> SceneType;
}
