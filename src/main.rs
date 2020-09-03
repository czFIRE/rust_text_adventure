use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const VERBOSE: bool = true;
#[derive(PartialEq, Debug)]
enum ReaderState {
    LoadingSeq,
    LoadingScene,
    LoadingChoices,
}

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
enum SceneType {
    Normal,
    Fight,
    End,
}

#[derive(Debug)]
struct Scene {
    scene_type: SceneType,
    scene_text: String,
    choices: Vec<String>,
}

impl Scene {
    fn from(scene_type: SceneType, scene_text: String, choices: Vec<String>) -> Self {
        //let res: Vec<String> = choices.iter().map(|s| s.to_string()).collect();
        Scene {
            scene_type,
            scene_text,
            choices,
        }
    }
}

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

fn get_user_action(max_choice: usize) -> usize {
    loop {
        let mut action: String = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("It's faulty mate, STDIN is kaput.");

        match action.trim().parse::<usize>() {
            Ok(num) => {
                if num > max_choice || num < 1 {
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

fn file_parser(scenes: &mut HashMap<String, Scene>, arguments: Vec<String>) -> std::io::Result<()> {
    let source_file = File::open(&arguments[1])?;
    let source_reader = BufReader::new(source_file);

    let mut reader_state = ReaderState::LoadingSeq;
    let mut scene_line: Vec<String> = Vec::new();
    let mut scene_text: String = String::new();
    let mut scene_choices: Vec<String> = Vec::new();

    for line in source_reader.lines() {
        let current_line = line?;

        //doesn't work for last
        if current_line.trim().is_empty() {
            let scene = Scene::from(
                SceneType::from_string(scene_line[2].as_ref()),
                scene_text.clone(),
                scene_choices.clone(),
            );
            scenes.insert(scene_line[1].to_string(), scene);
            reader_state = ReaderState::LoadingSeq;
            scene_line.clear();
            scene_text.clear();
            scene_choices.clear();
            continue;
        }

        if current_line.starts_with('#') {
            if VERBOSE {
                println!("{}", current_line);
            }
            continue;
        }

        println!("smt: {}", current_line);

        if current_line.starts_with(":>") {
            let curr_line = current_line.split_whitespace();
            scene_line = curr_line.map(String::from).collect();
            reader_state = ReaderState::LoadingScene;
            continue;
        }

        if current_line.chars().next().unwrap().is_numeric() {
            scene_text.push_str(&current_line);
            scene_text.push('\n');
            reader_state = ReaderState::LoadingChoices;

            let choice = current_line.split_whitespace().collect::<Vec<_>>();
            let val = String::from(choice[1]);
            scene_choices.push(val);
            continue;
        }

        //this can be done using match
        if reader_state == ReaderState::LoadingScene {
            scene_text.push_str(&current_line);
            scene_text.push('\n');
            continue;
        }
    }

    Ok(())
}
