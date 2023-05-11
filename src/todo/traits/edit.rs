use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;

use crate::state::write_to_file;
use super::super::enums::TaskStatus;
pub trait Edit {
    fn set_to_done(&self, name: &String, state: &mut Map<String, Value>){
        let item = state.insert(name.to_string(), json!(TaskStatus::DONE.stringify()));
        write_to_file("./state.json", state);        
        println!("{} is being set to done", name);
    }
    fn set_to_pending(&self, name: &String, state: &mut Map<String, Value>) {
        let item = state.insert(name.to_string(), json!(TaskStatus::PENDING.stringify()));
        write_to_file("./state.json", state); 
        println!("{} is being set to pending", name);
    }
}