fn main() {
    // Structs (regular definition)
    struct Point {
        x: f64,
        y: f64,
    }
    let point: Point = Point { x: 3.0, y: 4.0 };
    println!(
        "Struct: Point({}, {}) | bytes occupied: {}",
        point.x,
        point.y,
        std::mem::size_of_val(&point)
    );

    // Structs (tuple struct definition - anonymous fields)
    #[derive(Debug)] // this attribute allows the struct to be printed using {:?}
    struct Url(&'static str, &'static str);
    let url: Url = Url("https://example.com", "Example Domain");
    println!(
        "Struct: {:?} | fields: {} {} | bytes occupied: {}",
        url,
        url.0,
        url.1,
        std::mem::size_of_val(&url)
    );
}
