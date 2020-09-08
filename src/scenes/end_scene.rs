use super::{Scene, SceneType};

#[derive(Debug)]
pub struct EndScene {
    scene_type: SceneType,
    scene_text: String,
}

impl EndScene {
    /// Constructs new instance of Scene
    pub fn new_inst(scene_type: SceneType, scene_text: String) -> Box<dyn Scene> {
        Box::new(EndScene {
            scene_type,
            scene_text,
        })
    }
}

impl Scene for EndScene {
    fn playout(&mut self) -> String {
        println!("{}", self.scene_text);
        "".to_string()
    }

    fn get_scene_text(&self) -> String {
        self.scene_text.clone()
    }

    fn get_scene_type(&self) -> SceneType {
        self.scene_type.clone()
    }
}
