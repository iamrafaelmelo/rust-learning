fn main() {
    let mut mutable_number: u32 = 10;
    println!("[mutability] before mutation: {}", mutable_number);

    mutable_number = 20;
    println!("[mutability] after mutation: {}", mutable_number);

    {
        println!("[mutability] before mutation in another scope: {}", mutable_number);
        mutable_number = 30;
        println!("[mutability] after mutation in another scope: {}", mutable_number);
    }

    println!("[mutability] final muttation: {}", mutable_number);

    let _immutable_number: u32 = 5;
    // _immutable_number += 1; // error: cannot assign twice to immutable variable
}
