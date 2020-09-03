use std::collections::HashMap;
use std::env;

mod scenes;
use scenes::Scene; 
use scenes::{SceneType, io_interactions::*};

fn main() -> std::io::Result<()> {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() != 2 {
        panic!(
            "Wrong amount of arguments, expected 1, got {}. You need to specify the starting file.",
            arguments.len() - 1
        )
    }

    println!("{:?}", arguments[1]);

    let mut scenes: HashMap<String, Scene> = HashMap::new();

    file_parser(&mut scenes, arguments)?;

    //println!("{:?}", scenes);

    // here it is hardcoded, do smt about it
    game_loop(scenes, String::from("scene1"));

    Ok(())
}

fn game_loop(scenes: HashMap<String, Scene>, start: String) {
    let mut scene_counter: i32 = 0;
    let mut current_choice: String = start;

    loop {
        scene_counter += 1;
        //potentially unsafe unwrap, consider checking here
        let current_scene = scenes.get(&current_choice).unwrap();
        println!("{}", current_scene.scene_text);

        if current_scene.scene_type == SceneType::End {
            break;
        }

        let num = get_user_action(current_scene.choices.len());
        current_choice = current_scene.choices[num - 1].clone();
    }
    println!("{}", scene_counter);
}


