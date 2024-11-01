use clap::{command, Arg, Command};
use std::fs;

fn main() {
    let read_action = "read_file";
    let create_action = "create";
    let match_result = command!()
        .subcommand(
            Command::new(&read_action).arg(
                Arg::new("file_name")
                    .long("file-name")
                    .required(true)
                    .help("File name"),
            ),
        )
        .subcommand(
            Command::new(create_action)
                .arg(Arg::new("dir").long("dir").help("Directory name"))
                .arg(Arg::new("file").long("file").help("file name")),
        )
        .get_matches();

    let action = match_result.subcommand_name();
    println!("is read::: {}", action.unwrap());
    if action.unwrap() == read_action {
        println!("READ ACTION");
        let read_args = match_result.subcommand_matches(&read_action);
        match read_args {
            Some(val) => {
                let a = val.get_one::<String>("file_name").unwrap();
                let my_path = std::path::Path::new(&a);
                if my_path.exists() {
                    read_file(a);
                    return;
                } else {
                    println!("Fle not exist");
                }
            }
            None => println!("No data????"),
        }
    } else if action.unwrap() == create_action {
        let create_args = match_result.subcommand_matches(&create_action);
        match create_args {
            Some(val) => {
                let dir_res = val.get_one::<String>("dir");
                match dir_res {
                    Some(val) => {
                        create_directory(val);
                        return;
                    }
                    None => println!("not dir"),
                }

                let file_res = val.get_one::<String>("file");

                match file_res {
                    Some(path) => {
                        create_file(path, "Test again");
                    }
                    None => println!("not file"),
                }
            }
            None => println!("No data????"),
        }
    }
}

pub fn read_file(path: &String) {
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
