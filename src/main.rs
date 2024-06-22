use std::{thread,time,io};
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

fn dramatic_print(text: &str)
{
    for c in text.chars(){
        print!("{}",c);
        let _ = io::stdout().flush();
        let delay = time::Duration::from_millis(5);
        thread::sleep(delay);
    }
}

fn process_story(story: &HashMap<String, Vec<State>>)
{
    let mut game_active = true;
    let mut counter = 0;

    while game_active {
        let actions:&Vec<Action> = &story["state"][counter].action;
        
        //println!("{}", story["state"][counter].dialogue);
        dramatic_print(&story["state"][counter].dialogue);
        print!("> ");
        let _ = io::stdout().flush();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read command :(");
        let command = input.trim();

        let command_valid = command_valid(actions,command);

        if command == "exit"
        {
            game_active = false;
        }
        else if command_valid.0{
        //    println!("Command was valid");
            counter = command_valid.1 as usize;
        }
    }
}

fn main() {

    /* Load game */
    let mut file = File::open("assets/story.toml").expect("Failed to read file");
    let mut story_str = String::new();
    file.read_to_string(&mut story_str).expect("Failed to read story as string");
   
    //let story:Value = toml::from_str(&story_str).expect("Failed to deserialize toml");
    let story:HashMap<String, Vec<State>> = toml::from_str(&story_str).expect("Failed to deserialize toml");
    /* TODO, prettier printing */

    /*
    println!("---META START---");
    println!("{:?}", story);
    println!("----META END----");
    println!("");
    */
    process_story(&story);
}
