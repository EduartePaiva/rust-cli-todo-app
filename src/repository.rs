use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub content: String,
    pub done: bool,
}

const CURRENT_TODO: &str = "./current_todo.json";
const BACKUP_TODO: &str = "./backup_todo.json";

fn get_todo_items(path: &str) -> Vec<Todo> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
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

fn save_todo_items(path: &str, todos: Vec<Todo>) {
    let text = serde_json::to_string(&todos).expect("Error while Serialize todos");

    let mut file =
        File::create(path).expect(format!("error creating file for path: {}", path).as_str());
    write!(file, "{}", text).expect("error saving the file");
}

pub fn get_current_todo() -> Vec<Todo> {
    return get_todo_items(CURRENT_TODO);
}

pub fn save_current_todo(todos: Vec<Todo>) {
    return save_todo_items(CURRENT_TODO, todos);
}

pub fn get_backup_todo() -> Vec<Todo> {
    return get_todo_items(BACKUP_TODO);
}

pub fn save_backup_todo(todos: Vec<Todo>) {
    return save_todo_items(BACKUP_TODO, todos);
}
