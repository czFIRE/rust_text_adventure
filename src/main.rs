use std::collections::HashMap;
use std::env;

mod scenes;
use scenes::io_interactions::*;
use scenes::Scene;

fn main() -> std::io::Result<()> {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() != 2 {
        panic!(
            "Wrong amount of arguments, expected 1, got {}. You need to specify the starting file.",
            arguments.len() - 1
        )
    }

    //println!("{:?}", arguments[1]);

    let mut scenes: HashMap<String, Box<dyn Scene>> = HashMap::new();

    file_parser(&mut scenes, arguments)?;

    //println!("{:?}", scenes);

    // here it is hardcoded, do smt about it
    game_loop(scenes, String::from("scene1"));

    Ok(())
}

fn game_loop(mut scenes: HashMap<String, Box<dyn Scene>>, start: String) {
    let mut scene_counter: i32 = 0;
    let mut current_choice: String = start;

    loop {
        scene_counter += 1;
        //potentially unsafe unwrap, consider checking here
        let current_scene: &mut Box<dyn Scene> = scenes.get_mut(&current_choice).unwrap();

        match current_scene.playout().as_str() {
            "" => break,
            x => current_choice = x.to_string(),
        }
    }
    println!("{}", scene_counter);
}
