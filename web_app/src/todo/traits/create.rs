use serde_json::json;
use serde_json::value::Value;
use serde_json::Map;

use crate::state::write_to_file;
pub trait Create {
    fn create(&self, name: &String, status: &String, state: &mut Map<String, Value>) {
        state.insert(name.to_string(), json!(status));
        write_to_file("./state.json", state);
        println!("Item: {} is being created", name);
    }
}
