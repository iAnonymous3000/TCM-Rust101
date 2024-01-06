// A simple SHA256 password cracker that uses a dictionary attack.

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use sha2::{Sha256, Digest};
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Ensuring correct usage and arguments
    if args.len() != 2 {
        eprintln!("Invalid arguments!");
        eprintln!("Usage: {} <sha256sum>", args[0]);
        exit(1);
    }

    let wanted_hash = &args[1];
    let password_file = "src/rockyou.txt"; // Ensure this file path is correct and the file is accessible

    println!("Attempting to crack: {}!\n", wanted_hash);

    // Handling file opening with proper error message
    let password_list = match File::open(password_file) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Failed to open password file: {}", password_file);
            exit(1);
        }
    };

    let reader = BufReader::new(password_list);
    let mut attempts = 1;

    // Iterating through each line of the file
    for line in reader.lines() {
        let line = match line {
            Ok(pwd) => pwd,
            Err(_) => continue, // continue to next line if there's an error reading
        };
        let password = line.trim().to_owned().into_bytes();
        let password_hash = format!("{:x}", Sha256::digest(&password));

        // Check if the hash matches the wanted hash and print the status
        if &password_hash == wanted_hash {
            println!("Password hash found after {} attempts! {} hashes to {}!", attempts, std::str::from_utf8(&password).unwrap(), password_hash);
            exit(0);
        }
        attempts += 1;
    }

    println!("Password hash not found after {} attempts!", attempts);
}
