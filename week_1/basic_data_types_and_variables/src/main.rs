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

    
}
