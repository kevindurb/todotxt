fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_runs() {
        use crate::main;
        main();
    }
}
