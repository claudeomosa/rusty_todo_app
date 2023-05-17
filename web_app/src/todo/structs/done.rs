use crate::todo::traits::create::Create;
use crate::todo::traits::delete::Delete;
use crate::todo::traits::edit::Edit;
use crate::todo::traits::get::Get;

use super::super::enums::TaskStatus;
use super::base::Base;
#[derive(Debug)]
pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            name: input_title.to_string(),
            status: TaskStatus::DONE,
        };
        return Self { super_struct: base };
    }
}

impl Get for Done {}
impl Edit for Done {}
impl Create for Done {}
impl Delete for Done {}
