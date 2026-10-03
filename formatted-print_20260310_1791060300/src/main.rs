fn main() {
    // Replacement automatically (the arguments will be stringified)
    println!("My age is {}", 25);

    // Positional arguments (arguments start at 0)
    // NOTICE: Rust check the correct number of arguments in compile time
    println!("We can use positional arguments like this: {0}, {1}, {0}", "first", "second");

    // This will cause a compile-time error because the second argument is missing
    // println!("We can use positional arguments like this: {1}, {0}, {1}", "first");

    // Named arguments
    println!("My name is {name} and I am {age} years old.", name = "Etilândia", age = 30);

    // Formatting outputs
    println!("Decimal: {}", 255); // out: 255
    println!("Binary: {:b}", 255); // out: 11111111
    println!("Octal: {:o}", 255); // out: 377
    println!("Octal (with prefix): {:#o}", 255); // out: 0o377
    println!("Hexadecimal: {:x}", 255); // out: ff
    println!("Hexadecimal (uppercase): {:X}", 255); // out: FF
    println!("Hexadecimal (with prefix): {:#x}", 255); // out: 0xff
    println!("Lower expression (atom size - sci notation): {:e}", 0.0000000001); // out: 1e-10

    // Justifying text
    println!("Right justified: {:>10}", "text");
    println!("Left justified: {:<10}", "text");
    println!("Center justified: {:^10}", "text");

    // Padding numbers
    println!("Padded number (right): {:0<6}", 1);
    println!("Padded number (left): {:0>6}", 1);

    // Dynamic padding size (with named argument)
    println!("Dynamic width padding: {:>width$}", "RUST", width = 10);

    // Formatting floating-point numbers
    println!("Floating-point number: {:.2}", 3.14159); // out: 3.14
    println!("Floating-point number (scientific notation): {:.2e}", 3.14159); // out: 3.14e0
    println!("Floating-point number (scientific notation, uppercase): {:.2E}", 3.14159); // out: 3.14E0
}
