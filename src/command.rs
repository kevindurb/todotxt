pub struct Command {
    pub name: String,
    pub param: String,
}

impl Command {
    pub fn new(args: &Vec<String>) -> Command {
        let name = args[1].clone();
        let mut param = String::from("");

        if args.len() > 2 {
            param = args[2..].join(" ");
        }

        Command { name, param }
    }
}
