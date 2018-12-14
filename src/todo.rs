use crate::command;
use std::fs;

pub fn get_todo_pathname() -> String {
    String::from("/Users/kdurbin/todo.txt")
}

fn read_todo_file() -> String {
    let todo_pathname = get_todo_pathname();
    fs::read_to_string(&todo_pathname)
        .expect("Cannot read todo.txt")
}

fn write_todo_file(contents: &String) {
    let trimmed_contents = contents.trim();
    let todo_pathname = get_todo_pathname();
    fs::write(&todo_pathname, trimmed_contents)
        .expect("Cannot write to todo.txt");
}

pub fn list() {
    let todo_contents = read_todo_file();
    println!("{}", todo_contents);
}

pub fn add(command: &command::Command) {
    let new_todo = command.param.clone();
    let todo_contents = read_todo_file();

    let new_contents = [todo_contents, new_todo].join("\n");

    write_todo_file(&new_contents);
    list();
}

pub fn delete(command: &command::Command) {
    let idx_to_delete: usize = command.param.clone().parse().unwrap();
    let todo_contents = read_todo_file();
    let todo_lines: Vec<&str> = todo_contents.lines().collect();
    if todo_lines.len() > 0 {
        let last_line = todo_lines.len() - 1;
        let mut before_lines: Vec<&str> = Vec::new();
        let mut after_lines: Vec<&str> = Vec::new();

        if idx_to_delete <= 0 {
            after_lines = todo_lines[1..].to_vec();
        } else if idx_to_delete >= last_line {
            before_lines = todo_lines[..last_line].to_vec();
        } else {
            before_lines = todo_lines[..idx_to_delete].to_vec();
            after_lines = todo_lines[idx_to_delete + 1..].to_vec();
        }

        let before_content = before_lines.join("\n");
        let after_content = after_lines.join("\n");
        let new_content = format!("{}\n{}", before_content, after_content);
        write_todo_file(&new_content);
    }

    list();
}

pub fn help(command: &command::Command) {
    print!("Command {} not found", command.name);
}
