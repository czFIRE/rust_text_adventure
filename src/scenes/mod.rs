/// # Module Scenes
pub mod end_scene;
pub mod entities;
pub mod fight_scene;
pub mod io_interactions;
pub mod text_scene;

/// Definition of all possible types of scene
#[derive(PartialEq, Debug, Clone)]
pub enum SceneType {
    Normal,
    Fight,
    End,
}

impl SceneType {
    /// Converter from string to SceneType.
    /// Every unknown scene type is converted to End type
    ///
    /// # Example
    ///
    /// ```
    /// use self::SceneType;
    /// let end = "end";
    /// let normal = "normal";
    /// let fight = "fight";
    /// let text = "text";
    ///
    /// assert_eq!(SceneType::End, SceneType::from_string(end));
    /// assert_eq!(SceneType::Normal, SceneType::from_string(normal));
    /// assert_eq!(SceneType::Fight, SceneType::from_string(fight));
    /// assert_eq!(SceneType::End, SceneType::from_string(text));
    /// ```
    fn from_string(state: &str) -> Self {
        match state {
            "normal" => SceneType::Normal,
            "fight" => SceneType::Fight,
            "end" => SceneType::End,
            _ => SceneType::End,
        }
    }
}

#[test]
fn from_string() {
    let end = "end";
    let normal = "normal";
    let fight = "fight";
    let text = "text";

    assert_eq!(SceneType::End, SceneType::from_string(end));
    assert_eq!(SceneType::Normal, SceneType::from_string(normal));
    assert_eq!(SceneType::Fight, SceneType::from_string(fight));
    assert_eq!(SceneType::End, SceneType::from_string(text));
}

/// Trait for all Scenes
pub trait Scene {
    /// Implementation of main logic of each scene
    fn playout(&mut self) -> String;
    /// Getter for the text of the scene
    fn get_scene_text(&self) -> String;
    /// Getter for the type of the scene
    fn get_scene_type(&self) -> SceneType;
}
