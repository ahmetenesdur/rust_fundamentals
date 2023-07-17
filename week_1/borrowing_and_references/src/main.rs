// Borrowing and References

fn main() {
    // In Rust, Borrowing is a way to pass data to a function without taking ownership of it.
    // Borrowing is a way to pass data to a function without taking ownership of it.
    // References are a way to refer to some resource without taking ownership of it. References are immutable by default.

    // Immutable References are created using & symbol. Immutable references are read-only.
    let my_string = String::from("hello world");
    let _my_string_ref = &my_string; // Now my_string_ref is a reference to my_string variable. We can read my_string_ref but can't modify it.

    // Let's see example an immutable References in a function.
    fn print_string(s: &String) {
        println!("The value of s is: {}", s);
    }

    let example_string = String::from("hello world");
    print_string(&example_string); // We are passing a reference to example_string variable.

    // Mutable References are created using &mut symbol. Mutable references are read-write.
    // We can have only one mutable reference to a particular piece of data in a particular scope.
    // We can have multiple immutable references to a particular piece of data in a particular scope.
    let mut my_string = String::from("hello world");
    change_string(&mut my_string); // We are passing a mutable reference to my_string variable.
    println!("The value of my_string is: {}", my_string);

    fn change_string(s: &mut String) {
        s.push_str("!");
    }

    // Quick reference guide for borrowing rules:
    // 1. At any given time, you can have either one mutable reference or any number of immutable references.
    // 2. You cannot have both a mutable and an immutable reference to the same variable at the same time.
    // 3. References must always be valid and point to a valid memory location.
    // 4. References automatically expire at the end of their scope.

    // Dangling References are references that point to an invalid memory location.
    // Rust compiler will not allow us to create dangling references.
    fn return_reference(some_string: &String) -> &String {
        some_string
    } // This function will not compile because we are returning a reference to some_string variable which will be out of scope when the function returns.
}
