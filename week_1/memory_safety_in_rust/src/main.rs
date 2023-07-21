// Memory safety in Rust

fn main() {
    // Rust is a statically typed language, which means that it must know the types of all variables at compile time. The compiler can usually infer what type we want to use based on the value and how we use it.
    // In cases when many types are possible, such as when converting a String to a number, we must add a type annotation.
    // Rust is also a strongly typed language, which means that it will not automatically convert one type to another. We must explicitly convert types when we want to do so.

    // Why is Memory Safety Important?
    // Memory safety is important because it prevents bugs that can be difficult to track down and fix.

    // Understanding Variable Lifetimes in Rust for Memory Safety
    // Rust's memory safety is enforced at compile time, so we can be confident that our code is safe from memory bugs when it compiles successfully.

    // Understanding Variable Lifetimes in Rust for Memory Safety
    // 1. When you create a variable, Rust allocates memory for it, and the variable becomes valid.
    // 2. As the program runs, the variable can be used within its scope, and Rust ensures that no other part of the code can modify or access the memory it occupies in an unsafe way.
    // 3. Once the variable goes out of scope, Rust automatically deallocates its memory, making it unavailable for further use.
}
