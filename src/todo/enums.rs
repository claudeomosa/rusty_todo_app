#[derive(Debug)]
pub enum TaskStatus {
    PENDING,
    DONE,
}

impl TaskStatus{
    pub fn stringify(&self) -> String {
        match &self {
            &Self::PENDING => {String::from("PENDING")},
            &Self::DONE => {String::from("DONE")},
        }
    }
}