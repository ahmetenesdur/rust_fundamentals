// Ownership Concepts

fn main() {
    // String in Rust are dynamically allocated in memory.
    let s = String::from("hello");

    // Rust also has a second string type, the string slice. String slices are references to some part of a String. (String slices are also called string literals.)
    let slice = &s[0..2];
    println!("{}", slice);

    // Ownership, when create a value, you are the owner of it and you are responsible for managing its memory.
    let _my_string = String::from("hello, world!");

    // Important point: Rust is that it follows a 'move semantics' approach to memory management. This means that when a value is assigned to another variable, the original value is no longer valid.
    let _s1 = String::from("hello");
    let _s2 = _s1;

    // println!("{}, world!", s1); // This will throw an error because s1 is no longer valid.

    // Stack And Heap Memory
    // Stack: All data stored on the stack must have a known, fixed size.
    // Heap: Data with unknown size at compile time or a size that might change must be stored on the heap instead.
    let x = 5; // Stored on the stack
    let y = String::from("hello"); // Stored on the heap
    println!("x = {}, y = {}", x, y);

    // More about ownership
    // When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless the data has been moved to be owned by another variable.

    let x = 5; // stored on the stack
    let y = String::from("hello"); // stored on the heap
    let z = y; // ownership of y is moved to z
    println!("x = {}, z = {}", x, z);
}
