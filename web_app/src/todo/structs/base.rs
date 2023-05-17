use super::super::enums::TaskStatus;
use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct Base {
    pub name: String,
    pub status: TaskStatus,
}
