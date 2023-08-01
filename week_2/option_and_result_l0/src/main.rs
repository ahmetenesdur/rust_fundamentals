// Simple introduction to Option and Result

fn main() {
    // Introduction to Option
    // Option is an enum with two variants: Some and None (defined in the standard library)
    // Option<T> is a generic type, T is the type of the value that might be inside the Some variant

    // pub enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // Option<T> is so useful that it's included in the prelude; you don't need to bring it into scope explicitly
    // The Option<T> enum is still just a regular enum, and Some(T) and None are still variants of type Option<T>

    // Example of using Option
    fn find_square_root(x: f64) -> Option<f64> {
        if x < 0.0 {
            None
        } else {
            Some(x.sqrt())
        }
    }

    let x = 4.0;
    let y = find_square_root(x);

    match y {
        Some(f) => println!("Square root of {} is {}", x, f),
        None => println!("Cannot find square root of {}", x),
    }

    // Introduction to Result
    // Result is an enum with two variants: Ok and Err (defined in the standard library)
    // Result<T, E> is a generic type, T is the type of the value that might be inside the Ok variant
    // E is the type of the error that might be inside the Err variant

    // pub enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // Example of using Result
    fn divide(x: f64, y: f64) -> Result<f64, String> {
        if y == 0.0 {
            Err(String::from("Cannot divide by zero"))
        } else {
            Ok(x / y)
        }
    }

    let x = 4.0;
    let y = 0.0;
    let divide_result = divide(x, y);

    match divide_result {
        Ok(f) => println!("{} divided by {} is {}", x, y, f),
        Err(s) => println!("Error: {}", s),
    }

    // Combining Option and Result
    // Option and Result are often used together

    // Simulated database access function
    fn get_from_database(key: &str) -> Option<f64> {
        let database = vec![("base", Some(4.0)), ("height", Some(5.0))];
        for (k, v) in database {
            if k == key {
                return v;
            }
        }
        None
    }

    fn calculate_triangle_area(base: Option<f64>, height: Option<f64>) -> Result<f64, String> {
        match (base, height) {
            (Some(b), Some(h)) => {
                if b <= 0.0 || h <= 0.0 {
                    Err("Base and height must be positive".to_string())
                } else {
                    Ok(0.5 * b * h)
                }
            }
            (None, _) => Err("Base is missing".to_string()),
            (_, None) => Err("Height is missing".to_string()),
        }
    }

    let base = get_from_database("base");
    let height = get_from_database("height");
    let area = calculate_triangle_area(base, height);

    match area {
        Ok(f) => println!("Area of triangle is {}", f),
        Err(s) => println!("Error: {}", s),
    }
}
