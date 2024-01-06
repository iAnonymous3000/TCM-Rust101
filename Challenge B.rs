// A simple calculator that performs basic arithmetic operations on two user-provided numbers.

use std::io;

fn main() {
    // Retrieve first number from the user
    println!("Enter the first number:");
    let num1 = read_number();

    // Retrieve second number from the user
    println!("Enter the second number:");
    let num2 = read_number();

    // Perform calculations
    let sum = num1 + num2;
    let difference = num1 - num2;
    let product = num1 * num2;
    let quotient = if num2 != 0 {
        Some(num1 as f64 / num2 as f64)
    } else {
        None
    };

    // Display the results
    println!("Results:");
    println!("{} + {} = {}", num1, num2, sum);
    println!("{} - {} = {}", num1, num2, difference);
    println!("{} * {} = {}", num1, num2, product);
    match quotient {
        Some(div) => println!("{} / {} = {:.2}", num1, num2, div),
        None => println!("Division by zero is not allowed."),
    }
}

/// Reads a number from the user, retrying until a valid number is entered
fn read_number() -> i32 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid integer:"),
        }
    }
}
