use std::io::{self, Write};

fn main() {
    println!("Simple Calculator");
    println!("----------------");

    loop {
        let mut num1 = get_number("Enter first number: ");
        let operator = get_operator();
        let num2 = get_number("Enter second number: ");

        let result = match operator {
            '+' => num1 + num2,
            '-' => num1 - num2,
            '*' => num1 * num2,
            '/' => {
                if num2 != 0.0 {
                    num1 / num2
                } else {
                    println!("Error: Division by zero!");
                    continue;
                }
            }
            _ => {
                println!("Invalid operator!");
                continue;
            }
        };

        println!("Result: {} {} {} = {}", num1, operator, num2, result);

        print!("Do you want to continue? (y/n): ");
        io::stdout().flush().unwrap();
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).unwrap();
        if answer.trim().to_lowercase() != "y" {
            break;
        }
    }
}

fn get_number(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number!"),
        }
    }
}

fn get_operator() -> char {
    loop {
        print!("Enter operator (+, -, *, /): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let operator = input.trim().chars().next().unwrap_or(' ');
        if ['+', '-', '*', '/'].contains(&operator) {
            return operator;
        }
        println!("Please enter a valid operator!");
    }
}