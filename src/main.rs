use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Games: ");

    let secret_number = rand::thread_rng().gen_range(1..=10);
    println!("The secret number is: {}", secret_number);
    loop {
        let mut line = String::new();
        println!("Input your guess: ");
        io::stdin().read_line(&mut line).expect("Fail");
        print!("Your inputs is: {}", line);

        let line: u32 = match line.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match line.cmp(&secret_number) {
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
