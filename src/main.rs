mod todo;
use todo::structs::done::Done;
use todo::structs::pending::Pending;

use crate::todo::enums::TaskStatus;
use crate::todo::todo_factory;
use crate::todo::traits::get::Get;

fn main() {
    let pending_task = Pending::new("a pending task");
    let done_task = Done::new("a done task");

    let item_types = todo_factory("task name", TaskStatus::PENDING);
    

    println!("{:#?}", pending_task);
    println!("{:#?}", done_task.get(done_task.super_struct.name.as_str()));
    println!("{:#?}", item_types);



}
