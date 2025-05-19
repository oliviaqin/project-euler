mod problems;

use problems::problems::p001;
use problems::problems::p002;
use problems::problems::p003;

use std::io;

fn main() {
    println!("Welcome, running project euler code stuffs!");

    let problem_number: u16;

    loop {
        println!("Please input a number from 1 to 3.");

        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        problem_number = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        break;
    }

    match problem_number {
        1 => p001(),
        2 => p002(),
        3 => p003(),
        _ => println!("Please enger a number from 1 to 3."),
    }
}
