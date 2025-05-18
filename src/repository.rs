use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub content: String,
    pub done: bool,
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

    match serde_json::from_reader::<BufReader<File>, Vec<Todo>>(buffer) {
        Ok(todos) => {
            return todos;
        }
        Err(error) => {
            if error.is_eof() {
                println!("{}", error.to_string());
                return vec![];
            }
            panic!("{error}");
        }
    }
}

pub fn save_current_todo(todos: Vec<Todo>) {
    let text = serde_json::to_string(&todos).expect("Error while Serialize todos");

    let mut file = File::create(CURRENT_TODO)
        .expect(format!("error creating file for path: {}", CURRENT_TODO).as_str());
    write!(file, "{}", text).expect("error saving the file");
}
