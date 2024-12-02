use std::fs::File;
use std::io::Read;

pub fn read_content_from_file(file_name: &str) -> String {
    let file_path = format!("./input/{file_name}");
    let mut file = File::open(file_path);
    let mut contents = String::new();

    match file {
        Ok(mut file) => {
            file.read_to_string(&mut contents);
        }
        Err(e) => {
            println!("Error opening file {file_name}: {e}");
            println!("You're currently in: {}", std::env::current_dir().unwrap().display());
        }
    }

    contents
}