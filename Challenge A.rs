// Challenge A: Birthday Announcement Program
// This program announces the user's current age and the age they will be after their next birthday.

fn main() {
    // Global constant representing the value to increment the age by
    const BIRTHDAY_INCREMENT: i32 = 1;

    // User's name
    let my_name = "Your Name"; // TODO: Replace with your actual name

    // User's birth month/day
    let my_birthday = "Month/Day"; // TODO: Replace with your actual birth month/day

    // User's current age
    let current_age = 30; // TODO: Replace with your current age

    // User's age after their next birthday
    let new_age = current_age + BIRTHDAY_INCREMENT;

    // Print out the statement with the user's details
    println!(
        "My name is {} and I am {} years old. I will turn {} on {}",
        my_name, current_age, new_age, my_birthday
    );
}
