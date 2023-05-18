use serde::Serialize;
use std::vec::Vec;

use serde_json::value::Value;
use serde_json::Map;

use crate::state::{self, read_file};
use crate::todo::{enums::TaskStatus, todo_factory};
use crate::todo::{structs::base::Base, ItemTypes};
use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};

#[derive(Serialize)] //`Serialize` macro enables struct’s attributes to be serialized to JSON with the name of the attribute as the key
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_tems: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8,
}

impl ToDoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> ToDoItems {
        let mut pending_array_buffer = Vec::new();
        let mut done_array_buffer = Vec::new();

        for item in input_items {
            match item {
                ItemTypes::Pending(packed) => pending_array_buffer.push(packed.super_struct),
                ItemTypes::Done(packed) => done_array_buffer.push(packed.super_struct),
            }
        }

        let pending_count: i8 = pending_array_buffer.len() as i8;
        let done_count: i8 = done_array_buffer.len() as i8;

        return ToDoItems {
            pending_items: pending_array_buffer,
            done_tems: done_array_buffer,
            pending_item_count: pending_count,
            done_item_count: done_count,
        };
    }
    pub fn get_state() -> ToDoItems {
        let state: Map<String, Value> = read_file("./state.json");
        let mut array_buffer = Vec::new();
        for (key, value) in state {
            let status = TaskStatus::from_string(value.as_str().unwrap().to_string());
            let item: ItemTypes = todo_factory(&key, status);
            array_buffer.push(item)
        }
        return ToDoItems::new(array_buffer);
    }
}
impl Responder for ToDoItems {
    type Body = BoxBody;
    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
