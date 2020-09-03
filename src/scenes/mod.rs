pub mod io_interactions;
pub mod end_scene;
pub mod text_scene;
pub mod fight_scene;

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

#[derive(PartialEq, Debug)]
pub enum SceneType {
    Normal,
    Fight,
    End,
}

#[derive(Debug)]
pub struct Scene {
    pub scene_type: SceneType,  //maybe remove pub in favour of getter?
    pub scene_text: String,
    pub choices: Vec<String>,
}

impl Scene {
    /// Constructs new instance of Scene
    fn from(scene_type: SceneType, scene_text: String, choices: Vec<String>) -> Self {
        Scene {
            scene_type,
            scene_text,
            choices,
        }
    }
}

pub trait nevim {
    fn playout();
}