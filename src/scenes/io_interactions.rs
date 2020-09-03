use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;
use super::{Scene, SceneType};

#[derive(PartialEq, Debug)]
enum ReaderState {
    LoadingSeq,
    LoadingScene,
    LoadingChoices,
}

pub fn get_user_action(max_choice: usize) -> usize {
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

pub fn file_parser(scenes: &mut HashMap<String, Scene>, arguments: Vec<String>) -> std::io::Result<()> {
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
/*
        if current_line.starts_with('#') {
            if VERBOSE {
                println!("{}", current_line);
            }
            continue;
        }
*/
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