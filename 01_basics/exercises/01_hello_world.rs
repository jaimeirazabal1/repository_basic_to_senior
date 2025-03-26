// Exercise 1: Hello World and Basic Types
// Objectives:
// 1. Print "Hello, World!"
// 2. Work with basic types
// 3. Implement basic functions
// 4. Use string formatting

fn main() {
    // 1. Basic printing
    println!("Hello, World!");

    // 2. Variables and basic types
    let name = "Rust Learner";
    let age: u32 = 25;
    let is_learning = true;

    // 3. String formatting
    println!("My name is {}, I'm {} years old", name, age);
    println!("Am I learning Rust? {}", is_learning);

    // 4. Basic function calls
    let sum = add_numbers(10, 20);
    println!("Sum of 10 and 20 is: {}", sum);

    // 5. Basic type conversion
    let float_number = 42.5;
    let integer = float_number as i32;
    println!("Float: {}, Integer: {}", float_number, integer);
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

// TODO: Complete these exercises:
// 1. Create a function that converts Celsius to Fahrenheit
// 2. Create a function that calculates the area of a rectangle
// 3. Create a function that reverses a string
// 4. Implement basic pattern matching with a match expression