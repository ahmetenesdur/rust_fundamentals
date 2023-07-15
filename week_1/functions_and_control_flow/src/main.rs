// Functions and Control Flow

fn main() {
    let sum = sum(5, 6);
    println!("sum is {}", sum);

    let foo = foo(5);
    println!("foo is {}", foo);

    // If statements are similar to other languages.
    let x = 5;

    if x == 5 {
        println!("x is five!");
    } else if x == 6 {
        println!("x is six!");
    } else {
        println!("x is not five or six :(");
    }

    // While loops are similar to other languages.
    let mut x = 5;
    let mut done = false;

    while !done {
        x += x - 3;

        println!("while: {}", x);

        if x % 5 == 0 {
            done = true;
            println!("done!");
        }
    }

    // For loops are similar to other languages. They can be used to iterate over an Iterator.
    for x in 0..5 {
        println!("For loop: {}", x);
    }

    let numbers: [i32; 4] = [1, 2, 3, 4];
    for n in numbers {
        println!("Iterating over an array: {}", n);
    }

    // Loop is a keyword that allows you to loop until you explicitly break.
    let mut x = 10;
    loop {
        x += x - 3;

        println!("loop: {}", x);

        if x % 5 == 0 {
            break;
        }
    }

    // Match is similar javascript's switch statement but more powerful.
    let number = 2;
    match number {
        1 => println!("One!"),
        2 => println!("Two!"),
        3 => println!("Three!"),
        _ => println!("Something else!"), // _ is the default case
    }

    // Match arms can have code blocks too. The last expression in the block is the value returned by the arm.
    let result = match number {
        1 => "One!",
        2 => "Two!",
        3 => "Three!",
        _ => "Something else!",
    };

    println!("Match result: {}", result);
}

// Functions are declared using the fn keyword and the function's name must be snake case.
// The function's parameters go inside the parentheses. The return type, if any, goes after an arrow ->.
// The body of the function goes inside curly brackets {}.
fn sum(x: i32, y: i32) -> i32 {
    x + y
}

// The return type can be inferred by the compiler, so you don't always need to specify it.
fn foo(x: i32) -> i32 {
    x + 1
}

// Functions can be defined anywhere, even inside other functions.
fn inside() {
    fn bar(x: i32) -> i32 {
        x + 1
    }
}
