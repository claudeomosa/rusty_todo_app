#[allow(unused_variables, dead_code, unused_imports)]

mod todo;
mod state;
use std::env;
use state::{write_to_file, read_file};
use serde_json::value::Value;
use serde_json::{Map, json};
use todo::structs::done::Done;
use todo::structs::pending::Pending;

use crate::todo::enums::TaskStatus;
use crate::todo::todo_factory;
use crate::todo::traits::get::Get;

fn main() {
    let args: Vec<String> = env::args().collect();
    let status: &String = &args[1];
    let name: &String = &args[2];
    let mut state: Map<String, Value> = read_file("./state.json");
    // println!("Before: {:?}", state);
    state.insert(name.to_string(), json!(status));
    // println!("After: {:?}", state);
    write_to_file("./state.json", &mut state);


    let pending_task = Pending::new("a pending task");
    let done_task = Done::new("a done task");

    let item_types = todo_factory("task name", TaskStatus::PENDING);
    

    // println!("{:#?}", pending_task);
    println!("{:#?}", done_task.get(&String::from("beevs"), &state));
    // println!("{:#?}", item_types);


}
