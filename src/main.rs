use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use toml::{Value, de::Error};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug)]
struct State{
    dialogue: String,
    action: Vec<Action>,
}

#[derive(Deserialize, Debug)]
struct Action{
    action: String,
    next_state: u32,
}

fn main() {

    /* Load game */
    let mut file = File::open("assets/story.toml").expect("Failed to read file");
    let mut story_str = String::new();
    file.read_to_string(&mut story_str).expect("Failed to read story as string");
   
    //let story:Value = toml::from_str(&story_str).expect("Failed to deserialize toml");
    let story:HashMap<String, Vec<State>> = toml::from_str(&story_str).expect("Failed to deserialize toml");
    /* TODO, prettier printing */

    println!("---META START---");
    println!("{:?}", story);
    println!("----META END----");
    println!("");

    println!("{:?}", story["state"][0].dialogue);
    //println!("{:?}", story["state"][0]["dialogue"]);

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
