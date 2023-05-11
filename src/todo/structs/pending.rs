use super::base::Base;
use super::super::enums::TaskStatus;

#[derive(Debug)]
pub struct Pending {
    pub super_struct: Base,
}

impl Pending{
    pub fn new(input_title: &str) -> Self {
        let base = Base{
            name: input_title.to_string(),
            status: TaskStatus::PENDING,
        };
        return Self{super_struct: base}
    }
}