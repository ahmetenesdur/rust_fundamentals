// Hands-on Task - Implement a basic program that uses ownership concepts

fn main() {
    // Step 4: Create two String variables, string1 and string2, and initialize them with appropriate values.
    let string1: String = String::from("Hello, ");
    // rust symbol
    let string2: &str = "Rust! 🦀"; // string2 is a string slice (&str)

    // Step 5: Call the concatenate_strings function with references to string1 and string2 as arguments (using string slices).
    let concatenated_string: String = concatenate_strings(&string1, string2);

    // Step 6: Print the concatenated_string variable to the console.
    println!("{}", concatenated_string); // Hello, Rust! 🦀
}

// Step 1: Create a function called concatenate_strings that takes two string slices as arguments
// and returns a new String as the result of concatenating the two input strings.
fn concatenate_strings(string1: &str, string2: &str) -> String {
    // Step 2: Inside the concatenate_strings function, create a new String called result.
    let mut result: String = String::new();

    // Use the push_str() method to append the contents of the first input string slice.
    result.push_str(string1);

    // Followed by the second input string slice.
    result.push_str(string2);

    // Step 3: Return the result string from the function.
    result
}
