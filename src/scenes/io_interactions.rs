/*use crate::{
    scenes::entities::*,
    scenes::{end_scene, fight_scene, text_scene},
    scenes::{Scene, SceneType},
};*/

use super::{
    entities::*,
    {end_scene, fight_scene, text_scene}, {Scene, SceneType},
};

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Result};

/// Used for parser logic, mostly obsolete and could be replaced
#[derive(PartialEq, Debug)]
enum ReaderState {
    LoadingSeq,
    LoadingScene,
    LoadingChoices,
}

/// Gets numerical input from player and checks whether the number is from correct range
pub fn get_user_action(max_choice: usize, min_choice: usize) -> usize {
    loop {
        let mut action: String = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("It's faulty mate, STDIN is kaput.");

        match action.trim().parse::<usize>() {
            Ok(num) => {
                if num > max_choice || num < min_choice {
                    println!("Wrong choice, number out of choice scope. Try again");
                    continue;
                }
                return num;
            }
            Err(_) => {
                println!("This isn't a number. Try again");
                continue;
            }
        };
    }
}

/// Parses the input file specified in arguments from command line
pub fn file_parser(
    scenes: &mut HashMap<String, Box<dyn Scene>>,
    arguments: Vec<String>,
) -> std::io::Result<()> {
    let source_file = File::open(&arguments[1])?;
    let source_reader = BufReader::new(source_file);

    let mut reader_state = ReaderState::LoadingSeq;
    // probably obsolete and should be replaced with variables
    let mut scene_line: Vec<String> = Vec::new();
    let mut scene_text: String = String::new();
    let mut scene_choices: Vec<String> = Vec::new();

    let mut player: Player = Player::from(-1, -1, -1);
    let mut monsters: Vec<Monster> = Vec::new();

    for line in source_reader.lines() {
        let current_line = line?;

        // commentary line
        if current_line.starts_with('#') {
            continue;
        }

        // Empty lines serve as seperators for scenes and "give command" for new Scene to be generated
        // doesn't work for last
        if current_line.trim().is_empty() {
            let scene_type = SceneType::from_string(scene_line[2].as_ref());
            let scene: Box<dyn Scene> = create_scene(
                scene_type,
                scene_text.clone(),
                scene_choices.clone(),
                player.clone(),
                monsters.clone(),
            );

            scenes.insert(scene_line[1].to_string(), scene);

            reader_state = ReaderState::LoadingSeq;
            scene_line.clear();
            scene_text.clear();
            scene_choices.clear();

            continue;
        }

        // line that specifies name, type and other modifiers
        if current_line.starts_with(":>") {
            let curr_line = current_line.split_whitespace();
            scene_line = curr_line.map(String::from).collect();
            reader_state = ReaderState::LoadingScene;

            if scene_line[2] == "fight" {
                let (a, b) = load_fight_scene(scene_line[3].clone())?;
                player = a;
                monsters = b;
                scene_choices.push(scene_line[4].clone());
            }
            continue;
        }

        // line declaring choices
        if current_line.chars().next().unwrap().is_numeric() {
            scene_text.push_str(&current_line);
            scene_text.push('\n');
            reader_state = ReaderState::LoadingChoices;

            let choice = current_line.split_whitespace().collect::<Vec<_>>();
            let val = String::from(choice[1]);
            scene_choices.push(val);
            continue;
        }

        // line with text
        if reader_state == ReaderState::LoadingScene {
            scene_text.push_str(&current_line);
            scene_text.push('\n');
            continue;
        }
    }

    Ok(())
}

/// Returns new Scene based on scene type. Result is allocated on heap
fn create_scene(
    scene_type: SceneType,
    scene_text: String,
    scene_choices: Vec<String>,
    player: Player,
    monsters: Vec<Monster>,
) -> Box<dyn Scene> {
    match scene_type {
        SceneType::Normal => text_scene::TextScene::new_inst(scene_type, scene_text, scene_choices),
        SceneType::Fight => fight_scene::FightScene::new_inst(
            scene_type,
            scene_text,
            scene_choices,
            player,
            monsters,
        ),
        SceneType::End => end_scene::EndScene::new_inst(scene_type, scene_text),
    }
}

/// Parser for loading monsters from file, returns Player stats for scene and vector of monsters he will have to fight
fn load_fight_scene(path: String) -> Result<(Player, Vec<Monster>)> {
    let source_file = File::open(path)?;
    let source_reader = BufReader::new(source_file);

    let mut loaded_player: bool = false;

    let mut player: Player = Player::from(-1, -1, -1);
    let mut monsters: Vec<Monster> = Vec::new();

    for line in source_reader.lines() {
        let current_line = line?;

        // commentary line
        if current_line.starts_with('#') || current_line.trim().is_empty() {
            continue;
        }

        if !loaded_player && current_line.starts_with("player") {
            let split_line: Vec<String> =
                current_line.split_whitespace().map(String::from).collect();

            //should really check this and put into its own function
            let health: i32 = split_line[1].parse::<i32>().unwrap();
            let light: i32 = split_line[2].parse::<i32>().unwrap();
            let heavy: i32 = split_line[3].parse::<i32>().unwrap();

            player = Player::from(health, light, heavy);
            loaded_player = true;
            continue;
        }

        if loaded_player {
            let split_line: Vec<String> =
                current_line.split_whitespace().map(String::from).collect();
            let name: String = split_line[0].clone();
            let health: i32 = split_line[1].parse::<i32>().unwrap();
            let max_dmg: i32 = split_line[2].parse::<i32>().unwrap();
            let monster = Monster::from(name, health, max_dmg);
            monsters.push(monster);
            continue;
        } else {
            panic!("Wrong file format, you probably have wrong starting line");
        }
    }

    Ok((player, monsters))
}
