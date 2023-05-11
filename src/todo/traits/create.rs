use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;

use crate::state::write_to_file;
pub trait Create {
    fn create(&self, name: &String, status: &String, state: &mut Map<String, Value>){
        match state.insert(name.to_string(), json!(status)){
            Some(existing_item) => {
                println!("\n\nItem: {} already exixts", name)
            }
            None =>  {
                write_to_file("./state.json", state);
                println!("Item: {} is being created", name);
            }
        }
    }
}