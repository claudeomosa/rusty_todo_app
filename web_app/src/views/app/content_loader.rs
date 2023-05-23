use std::fs;

pub fn read_html_file(file_path: &str) -> String {
    let data: String  = fs::read_to_string(file_path).expect("unable  to read");
    return data
}