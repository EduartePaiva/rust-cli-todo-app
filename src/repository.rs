use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};

#[derive(Serialize, Deserialize)]
pub struct Todo {
    content: String,
    done: bool,
}

const CURRENT_TODO: &str = "./current_todo.json";

pub fn get_current_todo() -> Vec<Todo> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(CURRENT_TODO)
        .expect("Error opening current file");

    let buffer = BufReader::new(file);

    let todos: Vec<Todo> = serde_json::from_reader(buffer).expect("error descerializing file");

    return todos;
}

pub fn save_current_todo(todos: Vec<Todo>) {
    let text = serde_json::to_string(&todos).expect("Error while Serialize todos");

    let mut file = File::create(CURRENT_TODO)
        .expect(format!("error creating file for path: {}", CURRENT_TODO).as_str());
    write!(file, "{}", text).expect("error saving the file");
}
