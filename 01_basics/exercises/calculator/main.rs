// Importamos los módulos necesarios de la biblioteca estándar de Rust
// 'io' proporciona funcionalidades para entrada/salida
// 'self' permite usar el módulo io directamente
// 'Write' es un trait que permite escribir datos en un buffer
use std::io::{self, Write};

// Función principal que se ejecuta al iniciar el programa
fn main() {
    // Imprimimos el título de la calculadora
    println!("Simple Calculator");
    println!("----------------");

    // Bucle principal que se ejecuta hasta que el usuario decida salir
    loop {
        // Obtenemos el primer número del usuario
        let num1 = get_number("Enter first number: ");
        // Obtenemos el operador matemático
        let operator = get_operator();
        // Obtenemos el segundo número
        let num2 = get_number("Enter second number: ");

        // Utilizamos match para realizar la operación según el operador seleccionado
        let result = match operator {
            '+' => num1 + num2,    // Suma
            '-' => num1 - num2,    // Resta
            '*' => num1 * num2,    // Multiplicación
            '/' => {
                // Para la división, verificamos que no se divida por cero
                if num2 != 0.0 {
                    num1 / num2
                } else {
                    println!("Error: Division by zero!");
                    continue;  // Volvemos al inicio del bucle si hay división por cero
                }
            }
            _ => {
                println!("Invalid operator!");
                continue;  // Volvemos al inicio si el operador no es válido
            }
        };

        // Mostramos el resultado de la operación
        println!("Result: {} {} {} = {}", num1, operator, num2, result);

        // Preguntamos si el usuario desea continuar
        print!("Do you want to continue? (y/n): ");
        // Aseguramos que el prompt se muestre inmediatamente
        io::stdout().flush().unwrap();
        // Creamos una nueva cadena para almacenar la respuesta
        let mut answer = String::new();
        // Leemos la entrada del usuario
        io::stdin().read_line(&mut answer).unwrap();
        // Si la respuesta no es 'y' (ignorando mayúsculas/minúsculas), salimos del bucle
        if answer.trim().to_lowercase() != "y" {
            break;
        }
    }
}

// Función auxiliar para obtener un número del usuario
fn get_number(prompt: &str) -> f64 {
    loop {
        // Mostramos el prompt al usuario
        print!("{}", prompt);
        // Aseguramos que el prompt se muestre inmediatamente
        io::stdout().flush().unwrap();
        
        // Creamos una nueva cadena para almacenar la entrada
        let mut input = String::new();
        // Leemos la entrada del usuario
        io::stdin().read_line(&mut input).unwrap();
        
        // Intentamos convertir la entrada a un número decimal (f64)
        match input.trim().parse() {
            Ok(num) => return num,  // Si la conversión es exitosa, retornamos el número
            Err(_) => println!("Please enter a valid number!"),  // Si hay error, pedimos un número válido
        }
    }
}

// Función auxiliar para obtener el operador matemático
fn get_operator() -> char {
    loop {
        // Mostramos el prompt para el operador
        print!("Enter operator (+, -, *, /): ");
        // Aseguramos que el prompt se muestre inmediatamente
        io::stdout().flush().unwrap();
        
        // Creamos una nueva cadena para almacenar la entrada
        let mut input = String::new();
        // Leemos la entrada del usuario
        io::stdin().read_line(&mut input).unwrap();
        
        // Obtenemos el primer carácter de la entrada (el operador)
        let operator = input.trim().chars().next().unwrap_or(' ');
        // Verificamos si el operador es válido
        if ['+', '-', '*', '/'].contains(&operator) {
            return operator;  // Si es válido, lo retornamos
        }
        // Si no es válido, pedimos un operador válido
        println!("Please enter a valid operator!");
    }
}