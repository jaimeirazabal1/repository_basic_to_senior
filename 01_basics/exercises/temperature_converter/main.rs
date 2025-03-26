use std::io::{self, Write};

fn main() {
    println!("Temperature Converter");
    println!("-------------------");

    loop {
        println!("\nSelect conversion:");
        println!("1. Celsius to Fahrenheit");
        println!("2. Fahrenheit to Celsius");
        println!("3. Exit");

        let choice = get_choice();

        match choice {
            1 => celsius_to_fahrenheit(),
            2 => fahrenheit_to_celsius(),
            3 => break,
            _ => println!("Invalid choice! Please select 1, 2, or 3"),
        }
    }
}

fn get_choice() -> i32 {
    print!("Enter your choice (1-3): ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input.trim().parse().unwrap_or(0)
}

fn celsius_to_fahrenheit() {
    print!("Enter temperature in Celsius: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    if let Ok(celsius) = input.trim().parse::<f64>() {
        let fahrenheit = (celsius * 9.0/5.0) + 32.0;
        println!("{:.1}°C = {:.1}°F", celsius, fahrenheit);
    } else {
        println!("Invalid input! Please enter a number.");
    }
}

fn fahrenheit_to_celsius() {
    print!("Enter temperature in Fahrenheit: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    if let Ok(fahrenheit) = input.trim().parse::<f64>() {
        let celsius = (fahrenheit - 32.0) * 5.0/9.0;
        println!("{:.1}°F = {:.1}°C", fahrenheit, celsius);
    } else {
        println!("Invalid input! Please enter a number.");
    }
}