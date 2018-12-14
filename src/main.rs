use std::env;
use std::fs;
use std::path::Path;
mod command;
mod todo;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = command::Command::new(&args);

    let todo_pathname = todo::get_todo_pathname();
    if !Path::new(&todo_pathname).exists() {
        fs::write(&todo_pathname, "").expect("Cannot create todo.txt");
    }

    match command.name.as_ref() {
        "add" => todo::add(&command),
        "list" => todo::list(),
        "delete" => todo::delete(&command),
        _ => todo::help(&command),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_runs() {
        use crate::main;
        main();
    }
}
