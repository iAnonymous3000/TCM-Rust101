// A simple command-line quiz game.
// This program asks the user a series of questions and calculates their score based on the correct answers.

use std::io::{self, Write};

fn main() {
    let mut correct_answers = 0;
    let total_questions = 3;

    // Display a welcome message and instructions.
    println!("Welcome to our Quiz Game!");
    println!("Please select the correct answer for each question.");

    // Define the quiz questions, options, and answers.
    let questions = vec![
        (
            "1. What is the capital city of France?",
            vec!["A. London", "B. Paris", "C. Rome"],
            "B",
        ),
        (
            "2. What is the largest country in the world by area?",
            vec!["A. Russia", "B. Canada", "C. China"],
            "A",
        ),
        (
            "3. Who is credited with inventing the World Wide Web?",
            vec!["A. Bill Gates", "B. Tim Berners-Lee", "C. Steve Jobs"],
            "B",
        ),
    ];

    // Iterate through each question and collect answers.
    for (i, question) in questions.iter().enumerate() {
        ask_question(i + 1, question.0, &question.1);
        if get_and_check_answer(question.2) {
            correct_answers += 1;
        }
    }

    // Calculate and display the final score.
    let percentage = (correct_answers as f32 / total_questions as f32) * 100.0;
    println!("\nYou got {} out of {} questions correct ({:.2}%)", correct_answers, total_questions, percentage);
}

/// Presents a single question and its answer options to the user.
fn ask_question(number: usize, question: &str, options: &[&str]) {
    println!("\n{}. {}", number, question);
    for option in options {
        println!("{}", option);
    }
    print!("Your answer: ");
    io::stdout().flush().unwrap();  // Ensure the answer prompt is displayed before user input.
}

/// Retrieves the user's answer and checks it against the correct answer.
/// Returns true if the user's answer is correct, false otherwise.
fn get_and_check_answer(correct_answer: &str) -> bool {
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();  // Read the user's answer from standard input.

    if answer.trim().to_ascii_uppercase() == correct_answer {
        println!("Correct!");
        true
    } else {
        println!("Incorrect. The correct answer is {}.", correct_answer);
        false
    }
}
