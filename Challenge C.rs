use std::io;

fn main() {
    print_instructions();
    
    // Receive a value for X
    let x = read_number("Please give me a value for X.");

    // Receive a value for Y
    let y = read_number("Please give me a value for Y.");

    // Receive an operator
    let operator = read_operator();

    // Match operator and perform calculation
    match operator.as_str() {
        "+" => add(x, y),
        "-" => subtract(x, y),
        "*" => multiply(x, y),
        "/" => divide(x, y),
        _ => println!("Invalid operator. Exiting program."),
    }
}

fn print_instructions() {
    println!("Heath's Rust Calculator");
    println!("You must select two values (x and y) and an operator.");
}

// Math functions
fn add(x: i32, y: i32) {
    println!("The result of {} + {} = {}", x, y, x + y);
}

fn subtract(x: i32, y: i32) {
    println!("The result of {} - {} = {}", x, y, x - y);
}

fn multiply(x: i32, y: i32) {
    println!("The result of {} * {} = {}", x, y, x * y);
}

fn divide(x: i32, y: i32) {
    if y != 0 {
        let result = x as f64 / y as f64;
        println!("The result of {} / {} = {}", x, y, result);
    } else {
        println!("Cannot divide by zero!");
    }
}

// Function to read a number from the user, retrying until a valid number is entered
fn read_number(message: &str) -> i32 {
    loop {
        println!("{}", message);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number:"),
        }
    }
}

// Function to read an operator from the user
fn read_operator() -> String {
    println!("Choose an operator: +, -, *, /");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator).expect("Failed to read line");
    operator.trim().to_string()
}
