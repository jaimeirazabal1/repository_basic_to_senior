fn main() {
    // Basic types demonstration
    // i32 es un entero de 32 bits, lo que significa que puede almacenar valores enteros
    // en un rango de -2^31 a 2^31-1. El 32 indica la cantidad de bits utilizados para
    // representar el número.
    // 32 bits representan 4 bytes, y cada byte tiene 8 bits.
    // Por lo tanto, 32 bits = 4 bytes * 8 bits/byte = 32 bits.
    // Un entero de 32 bits puede almacenar valores en el rango de -2^31 a 2^31-1.
    // Esto significa que puede representar 2^32 (4,294,967,296) valores diferentes.
    // En términos de caracteres, si consideramos que cada carácter es un byte (8 bits),
    // entonces 32 bits pueden almacenar hasta 4 caracteres (32 bits / 8 bits por carácter).
    let integer: i32 = 42;

    // f64 es un número de punto flotante de 64 bits, lo que significa que puede almacenar
    // valores decimales con una mayor precisión en comparación con f32 (32 bits). El 64
    // indica la cantidad de bits utilizados para representar el número.
    // f64 es un número de punto flotante de 64 bits, lo que significa que puede almacenar
    // valores decimales con una mayor precisión en comparación con f32 (32 bits). El 64
    // indica la cantidad de bits utilizados para representar el número.
    // Es decir que 64 bits son 2^64.
    let float: f64 = 3.14;
    let boolean: bool = true;
    // Un char en Rust ocupa 4 bytes, es decir, 32 bits.
    // Un char en Rust ocupa 4 bytes, es decir, 32 bits.
    // Sin embargo, un char solo puede almacenar un único carácter Unicode.
    // No se pueden almacenar varios caracteres en un solo char.
    let character: char = 'A';
    // En Rust, los strings se manejan como objetos de la estructura String.
    // La estructura String proporciona una manera de trabajar con cadenas de texto que pueden cambiar de tamaño.
    // Aquí, estamos utilizando el método estático `from` de la estructura String para crear una nueva instancia de String.
    // El método `from` toma una cadena literal (&str) como argumento y devuelve una instancia de String.
    // Esto es útil porque las cadenas literales (&str) son inmutables y de tamaño fijo,
    // mientras que los objetos String son mutables y pueden cambiar de tamaño.
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
    // {:?} se utiliza para imprimir el array en el formato de depuración
    // {:?} es un formato de depuración que muestra el contenido del array en un formato legible.

    // {:?} es un especificador de formato que se utiliza para imprimir el array en el formato de depuración.
    // Si no se utiliza {:?}, no se puede ver el contenido del array correctamente y se producirá un error.
    // Es la forma recomendada de imprimir arrays y otros tipos complejos en Rust.
    println!("Array: {:?}", array);
}