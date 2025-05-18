use crate::repository::{Todo, get_current_todo, save_current_todo};

pub fn add(tasks: Vec<String>) {
    let mut todos = get_current_todo();

    for task in tasks {
        todos.push(Todo {
            content: task,
            done: false,
        });
    }
    save_current_todo(todos);
}

pub fn rm(index: usize) {
    let mut todos = get_current_todo();
    if index == 0 || index - 1 >= todos.len() {
        println!("index is out or range");
        return;
    }
    todos.remove(index + 1);
    save_current_todo(todos);
}
