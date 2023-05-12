#[derive(Debug)]
pub enum TaskStatus {
    PENDING,
    DONE,
}

impl TaskStatus {
    pub fn stringify(&self) -> String {
        match &self {
            &Self::PENDING => String::from("PENDING"),
            &Self::DONE => String::from("DONE"),
        }
    }
    pub fn from_string(input_string: String) -> Self {
        match input_string.as_str() {
            "DONE" => TaskStatus::DONE,
            "PENDING" => TaskStatus::PENDING,
            _ => panic!("Input {} not supported", input_string),
        }
    }
}
