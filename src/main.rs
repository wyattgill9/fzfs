use std::env;
use std::fs;
use std::path::Path;

fn list_files_recursive(dir: &Path, search_term: &str) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                list_files_recursive(&path, search_term);
            } else if let Some(name) = path.to_str() {
                if name.contains(search_term) {
                    println!("{}", name);
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

    list_files_recursive(Path::new("."), search_term);
}
