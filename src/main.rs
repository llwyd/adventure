use std::io;
use std::io::{Write};

fn main() {
    println!("Hello, world!");

    print!("> ");
    let _ = io::stdout().flush();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read command :(");
    let command = input.trim();

    match command{
        "hello" => println!("Hello!"),
        _ => {},
    }
}
