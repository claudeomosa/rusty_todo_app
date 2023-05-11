use std::fs::File;
use std::fs;
use std::io::Read;

use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;

pub fn read_file(file_name: &str) -> Map<String, Value> {
    let mut file = File::open(file_name.to_string()).unwrap();

    // read the content of `file` into a String `data`
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    // convert data from String to json, new type is `Value` and configure into Map 
    let json: Value = serde_json::from_str(&data).unwrap();
    let state: Map<String, Value> = json.as_object().unwrap().clone();
    return state
}

pub fn write_to_file(file_name: &str, state: &mut Map<String, Value>){
    // convert the Map back to a json using json! macro
    let new_data = json!(state);
    // if `filename` does not exist, a new one iscreated else, all content replaced with new_data converted from json to String
    fs::write(
        file_name.to_string(), new_data.to_string()
    ).expect("Error, Can't write to file!")
}