// A simple URL shortener: Accepts a long URL and provides a shortened version.

use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use rand::{distributions::Alphanumeric, Rng};

fn main() {
    let mapping_path = PathBuf::from("mapping.txt"); // Ensure this is the correct path for the mapping file
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <url>", args[0]);
        return;
    }

    let url = &args[1];

    // Check whether the URL should be shortened or expanded
    if url.starts_with("http://") || url.starts_with("https://") {
        // Generate and store short URL
        let short_url = generate_short_url(8);
        println!("Long URL: {}", url);
        println!("Short URL: {}", short_url);
        if let Err(e) = store_url_mapping(&mapping_path, &short_url, url) {
            eprintln!("{}", e);
            return;
        }
    } else {
        // Attempt to redirect to long URL
        match redirect_to_long_url(&mapping_path, url) {
            Ok(long_url) => println!("Redirecting to {}", long_url),
            Err(e) => eprintln!("{}", e),
        }
    }
}

/// Generates a random alphanumeric string of specified length
fn generate_short_url(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

/// Stores the mapping between short and long URL in a specified file
fn store_url_mapping(mapping_path: &PathBuf, short_url: &str, long_url: &str) -> Result<(), String> {
    let mut mapping_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(mapping_path)
        .map_err(|_| "Error opening mapping file.")?;

    let mapping = format!("{},{}\n", short_url, long_url);
    mapping_file
        .write_all(mapping.as_bytes())
        .map_err(|_| "Error writing to mapping file.")
}

/// Redirects to the long URL corresponding to the given short URL
fn redirect_to_long_url(mapping_path: &PathBuf, short_url: &str) -> Result<String, String> {
    let mapping_file = File::open(mapping_path).map_err(|_| "Error opening mapping file.")?;
    let reader = BufReader::new(mapping_file);

    for line in reader.lines() {
        let line = line.map_err(|_| "Error reading mapping file.")?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 2 && parts[0] == short_url {
            return Ok(parts[1].to_string());
        }
    }

    Err("Short URL not found.".to_string())
}
