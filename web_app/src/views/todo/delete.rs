use actix_web::{web, HttpResponse};
use serde_json::value::Value;
use serde_json::Map;

use crate::json_serialization::todo_item::ToDoItem;
use crate::jwt::JwToken;
use crate::processes::process_input;
use crate::state::read_file;
use crate::todo::{enums::TaskStatus, todo_factory};
use crate::json_serialization::todo_items::ToDoItems;

pub async fn delete(item: web::Json<ToDoItem>,  token: JwToken) -> HttpResponse {
    let state: Map<String, Value> = read_file("./state.json");
    let status: TaskStatus;
    match state.get(&item.name) {
        Some(result) => {
            status = TaskStatus::from_string(result.as_str().unwrap().to_string());
        },
        None => {
            return HttpResponse::NotFound().json(
                format!("{} not in state", &item.name)
            )
        }
    }
    let existing_item = todo_factory(item.name.as_str(), status.clone());
    process_input(existing_item, "delete".to_owned(), &state);
    return HttpResponse::Ok().json(ToDoItems::get_state())
}
