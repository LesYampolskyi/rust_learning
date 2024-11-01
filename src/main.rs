use std::fs;
fn main() {
    println!("Program starting");
    let directory_name = String::from("custom_data_2");
    let read_file_path = String::from("file01.txt");
    let text = "Test data";
    
    let mut path_file = String::from("./");

    path_file.push_str(&directory_name);
    path_file.push_str("/");
    path_file.push_str(&read_file_path);
    println!("Path is: {}", path_file);
   
    create_directory(&directory_name);
    create_file(&path_file, text);
    read_some_file(&path_file);
}

pub fn read_some_file(path: &String) {
    println!("Here data::: {}", path);
    let convert_bytes_to_string = |mut a: String, v: &u8| {
        let new_char = char::from(*v);
        a.push(new_char);
        a
    };

    let read_result = std::fs::read(path);
    if read_result.is_ok() {
        println!(
            "From file: {}",
            read_result
                .ok()
                .unwrap()
                .iter()
                .fold(String::from(""), convert_bytes_to_string)
        );
    } else {
        println!("Some problem to read file.");
    }
}

pub fn create_file(path: &String, text: &str) {
    let my_path = std::path::Path::new(path);
    if my_path.exists() {
        println!("=== File already exists");
        return;
    }
    println!("Create file, path: {}", path);
    _ = std::fs::write(path, text);
}

pub fn create_directory(path: &String) {
    let my_path = std::path::Path::new(path);
    if my_path.exists() {
        println!("Directory already exists! Skipping creating");
        return;
    }
    println!("========");
    let create_dir_result = fs::create_dir(path);
    if create_dir_result.is_ok() {
        println!("Directory created");
    } else {
        println!("Some problems {:?} ", create_dir_result.err())
    }
}
