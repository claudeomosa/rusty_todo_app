mod processes;
mod state;
mod todo;
use processes::process_input;
use serde_json::value::Value;
use serde_json::Map;
use state::read_file;
use std::env;

use crate::todo::enums::TaskStatus;
use crate::todo::todo_factory;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];
    let name: &String = &args[2];

    let state: Map<String, Value> = read_file("./state.json");
    let status: String;

    match &state.get(*&name) {
        Some(result) => {
            status = result.to_string().replace('\"', "");
        }
        None => {
            status = "pending".to_owned();
        }
    }

    let item = todo_factory(name, TaskStatus::from_string(status.to_uppercase()));

    process_input(item, command.to_string(), &state);
}
