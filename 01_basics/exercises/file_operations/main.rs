use std::fs::{self, File};
use std::io::{self, Write, Read};
use std::path::Path;

fn main() {
    loop {
        println!("\nFile Operations Menu");
        println!("------------------");
        println!("1. Create a file");
        println!("2. Write to a file");
        println!("3. Read from a file");
        println!("4. Delete a file");
        println!("5. List files in current directory");
        println!("6. Exit");

        match get_choice() {
            1 => create_file(),
            2 => write_to_file(),
            3 => read_file(),
            4 => delete_file(),
            5 => list_files(),
            6 => break,
            _ => println!("Invalid choice! Please select 1-6"),
        }
    }
}

fn get_choice() -> i32 {
    print!("Enter your choice (1-6): ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input.trim().parse().unwrap_or(0)
}

fn create_file() {
    print!("Enter filename: ");
    io::stdout().flush().unwrap();
    
    let mut filename = String::new();
    io::stdin().read_line(&mut filename).unwrap();
    let filename = filename.trim();

    match File::create(filename) {
        Ok(_) => println!("File created successfully!"),
        Err(e) => println!("Error creating file: {}", e),
    }
}

fn write_to_file() {
    print!("Enter filename: ");
    io::stdout().flush().unwrap();
    
    let mut filename = String::new();
    io::stdin().read_line(&mut filename).unwrap();
    let filename = filename.trim();

    print!("Enter content: ");
    io::stdout().flush().unwrap();
    
    let mut content = String::new();
    io::stdin().read_line(&mut content).unwrap();

    match File::create(filename) {
        Ok(mut file) => {
            match file.write_all(content.as_bytes()) {
                Ok(_) => println!("Content written successfully!"),
                Err(e) => println!("Error writing to file: {}", e),
            }
        }
        Err(e) => println!("Error opening file: {}", e),
    }
}

fn read_file() {
    print!("Enter filename: ");
    io::stdout().flush().unwrap();
    
    let mut filename = String::new();
    io::stdin().read_line(&mut filename).unwrap();
    let filename = filename.trim();

    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            match file.read_to_string(&mut content) {
                Ok(_) => println!("File content:\n{}", content),
                Err(e) => println!("Error reading file: {}", e),
            }
        }
        Err(e) => println!("Error opening file: {}", e),
    }
}

fn delete_file() {
    print!("Enter filename to delete: ");
    io::stdout().flush().unwrap();
    
    let mut filename = String::new();
    io::stdin().read_line(&mut filename).unwrap();
    let filename = filename.trim();

    match fs::remove_file(filename) {
        Ok(_) => println!("File deleted successfully!"),
        Err(e) => println!("Error deleting file: {}", e),
    }
}

fn list_files() {
    match fs::read_dir(".") {
        Ok(entries) => {
            println!("\nFiles in current directory:");
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("{}", entry.path().display());
                }
            }
        }
        Err(e) => println!("Error reading directory: {}", e),
    }
}