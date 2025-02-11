use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use std::io::Write;

fn list_files_recursive(dir: &Path, search_term: &str, results: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if dir_name != ".git" && dir_name != "target" {
                    list_files_recursive(&path, search_term, results);
                }
            } else if let Some(name) = path.to_str() {
                if name.contains(search_term) {
                    results.push(name.to_string());
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run <search_term>");
        return;
    }

    let search_term = &args[1];
    let mut results = Vec::new();

    list_files_recursive(Path::new("."), search_term, &mut results);

    if results.is_empty() {
        eprintln!("No matching files found.");
        return;
    }

    let fzf = Command::new("fzf")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn();

    match fzf {
        Ok(mut child) => {
            if let Some(stdin) = child.stdin.as_mut() {
                for result in &results {
                    writeln!(stdin, "{}", result).ok();
                }
            }

            if let Ok(output) = child.wait_with_output() {
                if let Ok(selected) = String::from_utf8(output.stdout) {
                    let selected_file = selected.trim();
                    if !selected_file.is_empty() {
                        Command::new("nvim")
                            .arg(selected_file)
                            .spawn()
                            .expect("Failed to open file");
                    }
                }
            }
        }
        Err(e) => eprintln!("Failed to launch fzf: {}", e),
    }
}
