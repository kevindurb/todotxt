use std::env;
mod todo;

fn args_to_command(args: Vec<String>) -> todo::TodoCommand {
    let name = args[1].clone();
    let mut param = String::from("");

    if args.len() > 2 {
        param = args[2..].join(" ");
    }

    match name.as_ref() {
        "add" => todo::TodoCommand::Add(param),
        "delete" => todo::TodoCommand::Delete(param.parse().unwrap()),
        "list" => todo::TodoCommand::List,
        _ => todo::TodoCommand::Help,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args_to_command(args);

    todo::run(command);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_runs() {
        use crate::main;
        main();
    }
}
