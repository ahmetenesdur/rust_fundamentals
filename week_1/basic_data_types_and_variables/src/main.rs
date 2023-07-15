fn main() {
    // Booleans are either true or false
    let _my_first_bool = true;
    let _my_second_bool: bool = false;

    // Integers are 8, 16, 32, 64, 128 bits wide and can be signed and unsigned (u8, i8, u16, i16, etc.)
    let _my_unsigned_8_bit_int: u8 = 255;
    let _my_signed_8_bit_int: i8 = -128;
    let _my_signed_128_bit_int: i128 = -170141183460469231731687303715884105728;

    // MIN and MAX values of integers can be accessed using the MIN and MAX constants
    let min_i32 = i32::MIN;
    let max_i32 = i32::MAX;
    println!(
        "The minimum value of i32 is {} and the maximum value is {}.",
        min_i32, max_i32
    );

    // Floating point numbers are 32 and 64 bits wide (f32, f64)
    let _my_32_bit_float: f32 = 3.14;
    let _my_64_bit_float: f64 = 3.14159265358979323846264338327950288;

    // Characters are 4 bytes wide and represent a Unicode Scalar Value
    let _my_char: char = '🦀';

    // Strings are two types: &str and String (String is a heap allocated string) and are UTF-8 encoded
    // str and String difference in Rust: https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str
    let _my_str: &str = "Hello, world!"; // &str is a string slice (a reference to a string)
    let _my_string: String = String::from("Hello, world!"); // String is a heap allocated string

    // Arrays are fixed length and contain elements of the same type
    let _my_array: [i32; 5] = [1, 2, 3, 4, 5]; // [type; length]

    let first_element: i32 = _my_array[0]; // Arrays are zero indexed
    let fifth_element = _my_array[4];

    println!(
        "The first element is {}, and the fifth element is {}.",
        first_element, fifth_element
    );

    // Slices are similar to arrays but their length is not known at compile time
    let _my_slice: &[i32] = &_my_array[0..3]; // Slices are a reference to a subset of an array

    println!("The slice is {:?}", _my_slice);

    // Tuples are fixed length and can contain elements of different types
    let _my_tuple: (i32, f64, char) = (1, 3.14, '🦀');

    let first_tuple_element = _my_tuple.0; // Tuples are zero indexed
    let second_tuple_element = _my_tuple.1;
    let third_tuple_element = _my_tuple.2;

    println!(
        "The first tuple element is {}, the second tuple element is {}, and the third tuple element is {}.",
        first_tuple_element, second_tuple_element, third_tuple_element
    );

    // Unit Type is a tuple with zero elements and is used to represent an empty value
    let _my_unit_type: () = (); // The unit type is written as ()

    println!("The unit type is {:?}", _my_unit_type);

    // Variables are immutable by default
    let _my_immutable_variable = 1;
    // _my_immutable_variable = 2; // This will throw an error

    // Variables can be made mutable by using the mut keyword
    let mut _my_mutable_variable = 1;
    _my_mutable_variable = 2; // This will not throw an error

    // Variables can be reassigned to a different type
    let _my_reassignable_variable = 1;
    let _my_reassignable_variable = "Hello, world!"; // This will not throw an error

    // Variables compiler can be automatically infer the type
    let _my_inferred_variable = 1; // The compiler will infer that this is an i32

    // Also supports shadowing (redeclaring a variable in the same scope)
    let _my_shadowed_variable = 1;
    let _my_shadowed_variable = 34.5; // This will not throw an error

    // Declare a new variable with the same name as an existing variable in a more nested scope
    let x = 1;
    let x = x + 1;
}
