use serde_json::value::Value;
use serde_json::Map;

pub trait Get {
    fn get(&self, name: &String, state: &Map<String, Value>) {
        let item: Option<&Value> = state.get(name);

        match item {
            Some(result) => {
                println!("\n\nItem: {}", name);
                println!("Status: {}\n\n", result);
            }
            None => println!("item: {} was not found", name),
        }
    }
}
