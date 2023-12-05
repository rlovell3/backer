use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: backer <target_file>");
        process::exit(1);
    }

    let target_file = PathBuf::from(&args[1]);

    let file_path = if target_file.is_relative() {
        Path::new(".").join(target_file)
    } else {
        target_file
    };

    if !file_path.exists() {
        eprintln!("Error: Target file '{}' does not exist.", file_path.display());
        process::exit(1);
    }

    let current_datetime = chrono::Local::now();
    let formatted_datetime = current_datetime.format("%Y%m%d_%H%M").to_string();
    let backup_filename = format!("{}.{}.backup", file_path.file_stem().unwrap().to_str().unwrap(), formatted_datetime);

    let backup_path = file_path.with_file_name(&backup_filename);
    println!("Backup path: {}", backup_path.display());
    
    match fs::copy(&file_path, &backup_path) {
        Ok(_) => println!("Backup successful: {}", backup_path.display()),
        Err(e) => eprintln!("Error during backup: {}", e),
    }
}