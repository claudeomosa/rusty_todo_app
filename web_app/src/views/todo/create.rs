use serde_json::value::Value;
use serde_json::Map;
use actix_web::HttpRequest;

use crate::todo::{todo_factory, enums::TaskStatus};
use crate::state::read_file;
use crate::processes::process_input;

pub async fn create(request: HttpRequest) -> String {
    let state: Map<String, Value> = read_file("./state.json");
    let name: String = request.match_info().get("name").unwrap().to_string(); //match_info, Returns a reference to the URL parameters container.
    let item = todo_factory(&name.as_str(), TaskStatus::PENDING);
    process_input(item, "create".to_string(), &state);
    return format!("{} created", name)
}
