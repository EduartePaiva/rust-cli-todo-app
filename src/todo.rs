use crate::repository::{
    Todo, get_backup_todo, get_current_todo, save_backup_todo, save_current_todo,
};

fn invalid_index(index: usize, length: usize) -> bool {
    if index == 0 || index - 1 >= length {
        println!("index is out or range");
        return true;
    }
    false
}

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
    if invalid_index(index, todos.len()) {
        return;
    }
    todos.remove(index - 1);
    save_current_todo(todos);
}

pub fn edit(index: usize, new_value: String) {
    let mut todos = get_current_todo();

    if invalid_index(index, todos.len()) {
        return;
    }

    todos[index].content = new_value;

    save_current_todo(todos);
}
pub fn list() {
    let todos = get_current_todo();

    for (i, todo) in todos.into_iter().enumerate() {
        println!("[{}] - {}, DONE: {}", i + 1, todo.content, todo.done);
    }
}
pub fn done(indexs: Vec<usize>) {
    let mut todos = get_current_todo();

    for i in indexs {
        if invalid_index(i, todos.len()) {
            continue;
        }
        todos[i - 1].done = true;
    }

    save_current_todo(todos);
}
pub fn reset() {
    let todos = get_current_todo();

    save_backup_todo(todos);
    save_current_todo(vec![]);
}
pub fn restore() {
    let todos = get_backup_todo();
    save_current_todo(todos);
}
pub fn sort() {
    let mut todos = get_current_todo();

    todos.sort_by(|a, b| a.done.cmp(&b.done));

    save_current_todo(todos);
}
