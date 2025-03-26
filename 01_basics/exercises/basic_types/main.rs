fn main() {
    // Basic types demonstration
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let boolean: bool = true;
    let character: char = 'A';
    let string: String = String::from("Hello, Rust!");

    // Printing different types
    println!("Hello, World!");
    println!("Integer: {}", integer);
    println!("Float: {}", float);
    println!("Boolean: {}", boolean);
    println!("Character: {}", character);
    println!("String: {}", string);

    // Tuple example
    let tuple: (i32, f64, char) = (1, 2.5, 'Z');
    println!("Tuple: ({}, {}, {})", tuple.0, tuple.1, tuple.2);

    // Array example
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", array);
}