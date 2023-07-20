// Clone Function

fn main() {
    // The Clone Trait and Clone Function
    // The Clone trait is used to explicitly make a copy of a value. It is used to make a copy of a value when the ownership of the original value is moved.
    let original = String::from("Hello, world!");
    let cloned = original.clone(); // The clone function is used to make a copy of a value.

    println!("original: {}, cloned: {}", original, cloned);

    // Using Clone with Borrowing and References
    // The clone function can be used with references and borrowing. When used with references, the clone function will make a copy of the value that the reference is pointing to.
    fn modify(s: &String) -> String {
        let mut cloned = s.clone(); // The clone function is used to make a copy of a value.
        cloned.push_str("modified"); // The cloned value is modified
        cloned // The cloned value is returned
    }

    let original = String::from("Hello, world!");
    let modified = modify(&original); // The modify function is called with a reference to the original value.
    println!("original: {}, modified: {}", original, modified);
    
    // In conclusion, the clone can be valuable working with borrowing and references. It can be used to make a copy of a value that is being borrowed. This can be useful when you want to modify a value that is being borrowed.
}
