use serde::Deserialize;

#[derive(Deserialize)]
pub struct ToDoItem {
    pub name: String,
    pub status: String,
}
