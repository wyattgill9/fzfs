use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// Recursively lists all files in the directory
fn list_files_recursive(dir: &Path, results: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if dir_name != ".git" && dir_name != "target" {
                    list_files_recursive(&path, results);
                }
            } else if let Some(name) = path.to_str() {
                results.push(name.to_string());
            }
        }
    }
}

/// Checks if a file name "fuzzily" matches a given query
fn fuzzy_match(filename: &str, query: &str) -> bool {
    let mut query_chars = query.chars();
    let mut current_char = query_chars.next();

    for c in filename.chars() {
        if Some(c) == current_char {
            current_char = query_chars.next();
            if current_char.is_none() {
                return true;
            }
        }
    }

    false
}

fn main() {
    let mut results = Vec::new();
    list_files_recursive(Path::new("."), &mut results);

    if results.is_empty() {
        eprintln!("No files found.");
        return;
    }

    let mut query = String::new();

    loop {
        print!("Search: {}_", query);
        io::stdout().flush().unwrap();

        let mut input = [0; 1];
        io::stdin().read_exact(&mut input).unwrap();
        let c = input[0] as char;

        if c == '\n' {
            break; // Enter key to exit
        } else if c == '\x08' || c == '\x7f' {
            // Backspace (handle different terminals)
            query.pop();
        } else {
            query.push(c);
        }

        print!("\x1B[2J\x1B[H"); // Clear screen
        println!("Search: {}", query);

        for file in &results {
            if fuzzy_match(file, &query) {
                println!("{}", file);
            }
        }
    }
}
