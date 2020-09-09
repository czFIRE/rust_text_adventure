/*use crate::{
    scenes::io_interactions::get_user_action,
    scenes::{Scene, SceneType}
};*/

use super::{
    io_interactions::get_user_action,
    {Scene, SceneType},
};

#[derive(Debug)]
pub struct TextScene {
    pub scene_type: SceneType,
    pub scene_text: String,
    pub choices: Vec<String>,
}

impl TextScene {
    /// Returns new instance of Scene
    pub fn new_inst(
        scene_type: SceneType,
        scene_text: String,
        choices: Vec<String>,
    ) -> Box<dyn Scene> {
        Box::new(TextScene {
            scene_type,
            scene_text,
            choices,
        })
    }
}

impl Scene for TextScene {
    /// Acts out the logic of choice text scene
    fn playout(&mut self) -> String {
        println!("{}", self.get_scene_text());

        let num = get_user_action(self.choices.len(), 1);

        self.choices[num - 1].clone()
    }

    fn get_scene_text(&self) -> String {
        self.scene_text.clone()
    }

    fn get_scene_type(&self) -> SceneType {
        self.scene_type.clone()
    }
}
