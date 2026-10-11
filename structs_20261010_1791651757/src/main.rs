fn main() {
    /*
    ---------------------------------------------------------------------------
    Structs (regular definition)
    ---------------------------------------------------------------------------
    */
    #[derive(Debug)]
    struct Point {
        x: f64,
        y: f64,
    }
    let point: Point = Point { x: 3.0, y: 4.0 };
    println!(
        "Struct (regular definition): Point({}, {}) | bytes occupied: {}",
        point.x,
        point.y,
        std::mem::size_of_val(&point)
    );

    /*
    ---------------------------------------------------------------------------
    Structs (tuple struct definition - anonymous fields)
    ---------------------------------------------------------------------------
    */
    #[derive(Debug)]
    struct Url(&'static str, &'static str, &'static str);
    let url: Url = Url("https", "example.com", "Example Domain");
    println!(
        "Struct (tuple struct definition): {:?} | fields: {} {} {} | bytes occupied: {}",
        url,
        url.0,
        url.1,
        url.2,
        std::mem::size_of_val(&url)
    );

    /*
    ---------------------------------------------------------------------------
    Struct composition/embedding (structs containing other structs)
    ---------------------------------------------------------------------------
    */
    #[derive(Debug)]
    struct Shape {
        width: f64,
        height: f64,
    }
    struct Rectangle {
        shape: Shape,
    }
    let rectangle: Rectangle = Rectangle {
        // struct instantiation expression
        shape: Shape {
            width: 5.0,
            height: 10.0,
        },
    };
    println!(
        "Struct composition: Rectangle({}, {}) | Type: {:?} | bytes occupied: {}",
        rectangle.shape.width,
        rectangle.shape.height,
        rectangle.shape,
        std::mem::size_of_val(&rectangle)
    );

    struct Square {
        shape: Shape,
    }
    let square: Square = Square {
        // struct instantiation expression
        shape: Shape {
            width: 5.0,
            height: 5.0,
        },
    };
    println!(
        "Struct composition: Square({}, {}) | Type: {:?} | bytes occupied: {}",
        square.shape.width,
        square.shape.height,
        square.shape,
        std::mem::size_of_val(&square)
    );

    /*
    ---------------------------------------------------------------------------
    Destructuring structs (named-fields)
    ---------------------------------------------------------------------------
    Notes:
    - The order of the fields in the struct definition does not matter.
    - The destructuring move data by default, the internal fields are moved out
    of the struct, so the struct is no longer usable after destructuring,
    depending on the type of the fields. If the fields are of types that implement
    the Copy trait, then the struct can still be used after destructuring.
    */

    // define a struct with named fields
    #[derive(Debug)]
    struct Result {
        success: bool,
        message: &'static str,
    }

    // instantiation the struct
    let result: Result = Result {
        success: true,
        message: "Operation completed successfully.",
    };

    // destructuring the struct into its fields (`result` still usable because &'static str and bool implement `Copy` trait)
    let Result { success, message } = result;
    println!(
        "Destructuring structs: Result({}, {}) | {} | bytes occupied: {}",
        success,
        message,
        result.message,
        std::mem::size_of_val(&result)
    );

    let Result {
        success: success_renamed,
        message: message_renamed,
    } = result;
    println!(
        "Destructuring structs (renamed fields): Result({}, {}) | {} | bytes occupied: {}",
        success_renamed,
        message_renamed,
        result.message, // usable
        std::mem::size_of_val(&result)
    );

    /*
    ---------------------------------------------------------------------------
    Destructuring structs (partial state problem)
    ---------------------------------------------------------------------------
    Rust prohibited strictly usage of any variable that be partial moved, because
    pieces of her does not exist anymore (in this example the point to String
    allocated on heap was already moved).

    Notes:
    - Fields that implements Copy trait will be copied to a new variable
    - Fields that not implement Copy trait will be moved to a new local variable
    - To solve this, we can destructuring borrowing the struct to keep the origihnal values
    */
    #[derive(Debug)]
    struct Config {
        host: String, // not implement Copy (will be moved to a new local variable)
        port: u16,    // implement Copy (will be copy to the new variables)
        debug: bool,  // implement Copy (will be copy to the new variables)
    }
    let config: Config = Config {
        host: String::from("localhost"),
        port: 8080,
        debug: true,
    };

    let Config { host, port, debug } = config;
    println!(
        "Destructuring structs (unsuable struct): {}, {}, {} | bytes occupied: {}",
        host,
        port,
        debug,
        // config, (if we try use `config`, compiler will show error message: `error[E0382]: borrow of partially moved value`
        std::mem::size_of_val(&result)
    );

    let config2: Config = Config {
        host: String::from("localhost"), // host was borrowed not moved
        port: 8080,
        debug: true,
    };
    let Config { host, port, debug } = &config2;
    println!(
        "Destructuring structs (unsuable struct): {}, {}, {} | {:#?} | bytes occupied: {}",
        host,
        port,
        debug,
        config2, // still usable
        std::mem::size_of_val(&result)
    );

    /*
    ---------------------------------------------------------------------------
    Destructuring structs (tuple definition)
    ---------------------------------------------------------------------------
    Notes:
    - In this case, the order of the fileds stricly matter (exactly order)
    */
    struct Color(i32, i32, i32);
    let red: Color = Color(255, 0, 0);
    let Color(r, g, b) = red;
    println!(
        "Destructuring structs (tuple def): {}, {}, {} | bytes occupied: {}",
        r,
        g,
        b,
        std::mem::size_of_val(&result)
    );
}
