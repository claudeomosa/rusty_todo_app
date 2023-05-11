use serde_json::Map;
use serde_json::value::Value;

use crate::state::write_to_file;
pub trait Delete {
    fn delete(&self, name: &String, state: &mut Map<String, Value>){
        let item: Option<Value> = state.remove(name);

        match item {
            Some(result) => {
                write_to_file("./state.json", state);
                println!("\n\nItem: {} is being deleted", name);
            },
            None => println!("item: {} was not found", name)
        }
    }
}