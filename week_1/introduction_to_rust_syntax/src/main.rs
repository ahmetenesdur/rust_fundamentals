fn main() {
    let message = "Hello, world!"; // let defines a variable

    let x: i32 = 42; // i32 is a 32-bit integer
    let pi: f64 = 3.14159; // f64 is a 64-bit float
    let is_rust_fun: bool = true; // bool is a boolean
    let letter_a: char = 'a'; // char is a character

    // Functions are defined with fn
    fn add(x: i32, y: i32) -> i32 {
        // -> i32 is the return type
        x + y // no semicolon means return
    } // no semicolon means end of function

    let x = 42; // Shadowing is allowed in Rust

    if x >= 0 {
        // if statements are the same as C (no parens)
        println!("x is non-negative"); // println! is a macro (not a function)
    } else {
        println!("x is negative");
    }

    let mut i = 1; // mut makes a variable mutable

    while i <= 5 { // while loops are the same as C
        println!("{}", i); // {} is a placeholder for a variable
        i += 1; // no ++ or -- in Rust
    } 
}
