fn main() {
    // Arrays (allocated on the stack)
    let numbers_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!(
        "Array (fixed size): {:?} | len: {} | bytes occupied: {}",
        numbers_array,
        numbers_array.len(),
        std::mem::size_of_val(&numbers_array)
    );

    let matrix_fixed_size: [[i32; 3]; 10] = [[0; 3]; 10];
    println!(
        "Array (2D array 10x3 with all elements initialized to 0): {:?} | len: {} | bytes occupied: {}",
        matrix_fixed_size,
        matrix_fixed_size.len(),
        std::mem::size_of_val(&matrix_fixed_size)
    );

    let empty_array: [i32; 0] = [];
    println!(
        "Array (empty): {:?} | len: {} | bytes occupied: {}",
        empty_array,
        empty_array.len(),
        std::mem::size_of_val(&empty_array)
    );

    // Vectors (allocated on the heap)
    let numbers_vector: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!(
        "Vector (dynamic array): {:?} | len: {} | bytes occupied: {}",
        numbers_vector,
        numbers_vector.len(),
        std::mem::size_of_val(&numbers_vector)
    );

    // Slices (borrow an array or vector, they are views into a slice)
    let numbers_slice: &[i32] = &numbers_vector[1..4]; // get elements at index 1 to 3 (exclusive)
    println!(
        "Slice (view into a vector): {:?} | len: {} | bytes occupied: {}",
        numbers_slice,
        numbers_slice.len(),
        std::mem::size_of_val(&numbers_slice)
    );

    let empty_slice: &[i32] = &[];
    println!(
        "Slice (empty): {:?} | len: {} | bytes occupied: {}",
        empty_slice,
        empty_slice.len(),
        std::mem::size_of_val(&empty_slice)
    );

    // Tuples
    let person: (&str, &str, i32) = ("Alice", "alice@email.com", 30);
    println!(
        "Person: {:?} | bytes occupied: {}",
        person,
        std::mem::size_of_val(&person)
    );

    // String (common string type, it has ownership and is allocated on the heap)
    let phrase: String = String::from("Lorm ipsum dolor sit amet, consectetur adipiscing elit.");
    println!(
        "String: {} | len: {} | bytes occupied: {}",
        phrase,
        phrase.len(),
        std::mem::size_of_val(&phrase)
    );
}
