fn main() {
    // ===================================
    // SCOPE
    // ===================================

    // this variable lives in main
    let main_lived_message: &str = "world!";
    println!("[scope] main scope message: {}", main_lived_message);

    {
        // this variable exists only in this scope
        // try handle this var out this scope occurs and error
        let short_lived_message: &str = "Hello, scope!";
        println!("[scope] short lived message: {}", short_lived_message)
    }

    // ===================================
    // SHADOWING
    // ===================================
    let shadowed_number = 1;

    {
        println!("[shadowing] before being shadowed: {}", shadowed_number);
        let shadowed_number = "abc";
        println!("[shadowing] shadowed in inner block: {}", shadowed_number);
    }

    println!("[shadowing] outside inner block: {}", shadowed_number);
    let shadowed_number: i32 = 2;
    println!("[shadowing] shadowed in outer block: {}", shadowed_number);
}
