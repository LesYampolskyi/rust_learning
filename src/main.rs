use csv::{self, Reader};
use std::{path::Path, result};

fn main() {
    let file_name = "file_test.csv";
    let result = Reader::from_path(file_name);

    if result.is_err() {
        println!("Failed to read a csv file. Some error, lilbro");
        std::process::exit(9);
    }

    println!("After error handling");

    let mut my_reader =  result.unwrap();

    for record in my_reader.records() {
        println!("====++++: {:?}", record);
        println!("====++++: {}", record.unwrap().get(0).unwrap());
    }


    // println!("Here we are!");

    // if let Err(e) = read_csv(String::from("file_test.csv")) {
    //     eprintln!("Error, lilbro: {}", e);
    // };
}

fn read_csv(filename: String) -> Result<(), csv::Error> {
    let path = Path::new(&filename);

    if path.exists() {
        println!("From read_csv() function...");
        let mut rcsv = csv::Reader::from_path(path)?;

        for result in rcsv.records() {
            let record = result?;
            println!("Result ===> {:?} ", record)
        }
    } else {
        println!("File not exists");
    }

    Ok(())
}
