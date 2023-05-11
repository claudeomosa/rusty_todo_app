use serde_json::value::Value;
use serde_json::Map;

use crate::state::write_to_file;
pub trait Delete {
    fn delete(&self, name: &String, state: &mut Map<String, Value>) {
        state.remove(name);
        write_to_file("./state.json", state);
        println!("\n\nItem: {} is being deleted", name);
    }
}
