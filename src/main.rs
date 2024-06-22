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

fn command_valid(actions:&Vec<Action>, command: &str) -> (bool, u32)
{
    let mut found = false;
    let mut next_state = 0;
    for action in actions{
        if action.action == command{
            found = true;
            next_state = action.next_state;
        }
    }
    
    (found, next_state)
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

    //println!("{:?}", story["state"][0].dialogue);
    let mut story_counter = 0;
    println!("{}", story["state"][story_counter].dialogue);
    let actions:&Vec<Action> = &story["state"][story_counter].action;
    println!("Available Actions: {:?}", *actions);

    print!("> ");
    let _ = io::stdout().flush();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read command :(");
    let command = input.trim();

    let command_valid = command_valid(actions,command);

    if command_valid.0{
    //    println!("Command was valid");
        story_counter = command_valid.1 as usize;
    }
    println!("{}", story["state"][story_counter].dialogue);
}
