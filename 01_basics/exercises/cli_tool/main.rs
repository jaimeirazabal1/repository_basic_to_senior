use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    match args[1].as_str() {
        "greet" => handle_greet(&args),
        "calc" => handle_calc(&args),
        "reverse" => handle_reverse(&args),
        "count" => handle_count(&args),
        "help" => print_usage(),
        _ => {
            println!("Unknown command: {}", args[1]);
            print_usage();
            process::exit(1);
        }
    }
}

fn print_usage() {
    println!("Simple CLI Tool");
    println!("Usage:");
    println!("  greet <name>           - Greet a person");
    println!("  calc <num1> <op> <num2> - Simple calculator");
    println!("  reverse <text>         - Reverse a string");
    println!("  count <text>           - Count characters");
    println!("  help                   - Show this help");
}

fn handle_greet(args: &[String]) {
    if args.len() < 3 {
        println!("Please provide a name!");
        process::exit(1);
    }
    println!("Hello, {}!", args[2]);
}

fn handle_calc(args: &[String]) {
    if args.len() < 5 {
        println!("Please provide two numbers and an operator!");
        process::exit(1);
    }

    let num1: f64 = args[2].parse().unwrap_or_else(|_| {
        println!("First argument must be a number!");
        process::exit(1);
    });

    let num2: f64 = args[4].parse().unwrap_or_else(|_| {
        println!("Third argument must be a number!");
        process::exit(1);
    });

    let result = match args[3].as_str() {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("Cannot divide by zero!");
                process::exit(1);
            }
            num1 / num2
        }
        _ => {
            println!("Invalid operator! Use +, -, *, or /");
            process::exit(1);
        }
    };

    println!("{} {} {} = {}", num1, args[3], num2, result);
}

fn handle_reverse(args: &[String]) {
    if args.len() < 3 {
        println!("Please provide text to reverse!");
        process::exit(1);
    }
    let reversed: String = args[2].chars().rev().collect();
    println!("{}", reversed);
}

fn handle_count(args: &[String]) {
    if args.len() < 3 {
        println!("Please provide text to count!");
        process::exit(1);
    }
    println!("Character count: {}", args[2].len());
}