use super::todo::structs::done::Done;
use super::todo::structs::pending::Pending;
use super::todo::traits::create::Create;
use super::todo::traits::delete::Delete;
use super::todo::traits::edit::Edit;
use super::todo::traits::get::Get;
use super::todo::ItemTypes;
use serde_json::value::Value;
use serde_json::Map;

fn process_pending(item: Pending, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();
    match command.as_str() {
        "get" => item.get(&item.super_struct.name, &state),
        "create" => item.create(
            &item.super_struct.name,
            &item.super_struct.status.stringify(),
            &mut state,
        ),
        "edit" => item.set_to_done(&item.super_struct.name, &mut state),
        _ => println!("command: {} not supported", command),
    }
}
fn process_done(item: Done, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();
    match command.as_str() {
        "get" => item.get(&item.super_struct.name, &state),
        "edit" => item.set_to_pending(&item.super_struct.name, &mut state),
        "delete" => item.delete(&item.super_struct.name, &mut state),
        _ => println!("command: {} not supported", command),
    }
}
pub fn process_input(item: ItemTypes, command: String, state: &Map<String, Value>) {
    match item {
        ItemTypes::Pending(item) => process_pending(item, command, state),
        ItemTypes::Done(item) => process_done(item, command, state),
    }
}
