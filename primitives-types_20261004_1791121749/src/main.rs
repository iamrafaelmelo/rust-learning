fn main() {
    // ========================================================================
    // SCALAR TYPES
    // ========================================================================

    // Integers (unsigned)
    let a: u8 = 255; // 8-bit unsigned integer
    let b: u16 = 65535; // 16-bit unsigned integer
    let c: u32 = 4294967295; // 32-bit unsigned integer
    let d: u64 = 18446744073709551615; // 64-bit unsigned integer
    println!("Unsigned Integers: a = {a}, b = {b}, c = {c}, d = {d}");

    // Integers (signed)
    let e: i8 = -128; // 8-bit signed integer
    let f: i16 = -32768; // 16-bit signed integer
    let g: i32 = -2147483648; // 32-bit signed integer
    let h: i64 = -9223372036854775808; // 64-bit signed integer
    println!("Signed Integers: e = {e}, f = {f}, g = {g}, h = {h}");

    // Integers (sizes based on machine architecture)
    let i: isize = 5; // Signed integer (4 bytes on 32-bit systems, 8 bytes on 64-bit systems)
    let j: usize = 10; // Unsigned integer (4 bytes on 32-bit systems, 8 bytes on 64-bit systems)
    println!(
        "Architecture-based Integers: i = {i} ({} bytes size), j = {j} ({} bytes size)",
        std::mem::size_of_val(&i),
        std::mem::size_of_val(&j)
    );

    // Integers (with suffixes)
    let k = 100i32;
    let l = 1000u64;
    let m = 127i8;
    println!(
        "Integers with Suffixes: k = {k} (type: {:?}), l = {l} (type: {:?}), m = {m} (type: {:?})",
        std::any::type_name_of_val(&k),
        std::any::type_name_of_val(&l),
        std::any::type_name_of_val(&m)
    );

    // Floating-point numbers
    let pi: f32 = 3.14159; // 32-bit floating-point number
    let euler: f64 = 2.718281828459045; // 64-bit floating-point number
    println!("Floating-point Numbers: pi = {pi}, euler = {euler}");

    // Boolean
    let is_active: bool = true;
    println!("Boolean: is_active = {is_active}");

    // Character (print with multline formatting)
    let letter: char = 'A';
    let chinese_character: char = '生';
    let arabic_character: char = 'ع';
    let smile_emoji: char = '😀';
    println!(
        r#"
        Character: letter = {letter} (size: {letter_size} bytes),
        chinese_character = {chinese_character} (size: {chinese_character_size} bytes)
        arabic_character = {arabic_character} (size: {arabic_character_size} bytes),
        smile_emoji = {smile_emoji} (size: {smile_emoji_size} bytes)
        "#,
        letter = letter,
        letter_size = std::mem::size_of_val(&letter),
        chinese_character = chinese_character,
        chinese_character_size = std::mem::size_of_val(&chinese_character),
        arabic_character = arabic_character,
        arabic_character_size = std::mem::size_of_val(&arabic_character),
        smile_emoji = smile_emoji,
        smile_emoji_size = std::mem::size_of_val(&smile_emoji)
    );

    // String slices (with a string literal)
    // Good for cases where we want to reference a string without taking ownership of it, and the string is known at compile time.
    let message: &'static str = "Hello, Rust!"; // point into the binary read-only section, which is valid for the entire lifetime of the program
    println!(
        "String Slice: message = {message} (size: {} bytes)",
        std::mem::size_of_val(&message)
    );

    let greeting: &str = "Hello, Rust!"; // compiler infers 'static
    println!(
        "String Slice: greeting = {greeting} (size: {} bytes)",
        std::mem::size_of_val(&greeting)
    );

    // Unit (empty tuple)
    let unit: () = ();
    println!("Unit: tuple = {:?}", unit);
}
