use actix_web::{web, HttpResponse};
use serde_json::value::Value;
use serde_json::Map;
use crate::jwt::JwToken;

use crate::state::read_file;

use crate::json_serialization::{todo_item::ToDoItem, todo_items::ToDoItems};
use crate::processes::process_input;
use crate::todo::{enums::TaskStatus, todo_factory};

pub async fn edit(item: web::Json<ToDoItem>, token: JwToken) -> HttpResponse //item will be extracted from the body of the request, serialized, and constructed as the ToDoItem struct
{
    println!("here is the message in the token: {}", token.message);
    let state: Map<String, Value> = read_file("./state.json");
    let status: TaskStatus;
    match &state.get(&item.name) {
        Some(result) => {
            status = TaskStatus::new(result.as_str().unwrap());
        }
        None => {
            return HttpResponse::NotFound().json(format!("{} not found in state", &item.name))
        }
    }
    let existing_item = todo_factory(item.name.as_str(), status.clone());
    if &status.stringify() == &TaskStatus::from_string(item.status.as_str().to_string()).stringify()
    {
        return HttpResponse::Ok().json(ToDoItems::get_state());
    }
    process_input(existing_item, "edit".to_owned(), &state);
    return HttpResponse::Ok().json(ToDoItems::get_state());
}
