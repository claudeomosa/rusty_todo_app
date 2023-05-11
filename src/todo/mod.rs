pub mod enums;
pub mod structs;
pub mod traits;

use enums::TaskStatus;
use structs::done::Done;
use structs::pending::Pending;

#[derive(Debug)]
pub enum ItemTypes{
    Pending(Pending),
    Done(Done),
}

pub fn todo_factory(name: &str, status: TaskStatus) -> ItemTypes {
    match status {
        TaskStatus::DONE => {
            ItemTypes::Done(Done::new("a new done task"))
        },
        TaskStatus::PENDING => {
            ItemTypes::Pending(Pending::new("a new pending task"))
        }
    }
}
