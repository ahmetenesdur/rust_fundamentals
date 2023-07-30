// Structs

fn main() {
    // Structs are similar to tuples, but each value has a name and can be of a different type from the others.
    // Structs are useful when you want to give a group of values a name and to refer to the group as a whole instead of individually.
    struct Book {
        title: String,
        author: String,
        pages: u32,
        publication_year: u32,
    }

    // Creating Instances of Structs
    // To use a struct after we’ve defined it, we create an instance of that struct by specifying concrete values for each of the fields.
    struct Point {
        x: f32,
        y: f32,
    }

    let p = Point { x: 10.0, y: 20.0 };
    println!("p.x = {}", p.x);
    println!("p.y = {}", p.y);

    let another_point = Point { x: 5.0, ..p }; // ..p means use the rest of the values from p
    println!("another_point.x = {}", another_point.x);
    println!("another_point.y = {}", another_point.y);

    // Accessing Struct Fields
    // We can access the fields of an instance using dot notation.
    struct Person {
        name: String,
        age: u8,
    }

    let mut alice = Person {
        name: String::from("Alice"),
        age: 20,
    };

    println!("person.name = {}", alice.name); // Accessing the name field of the person struct
    println!("person.age = {}", alice.age);

    alice.age += 1; // Changing the value of the age field
    println!("Updated age = {}", alice.age);

    // Functions and Structs
    // We can also define functions that take structs as parameters. This is useful when we want to encapsulate some functionality with a struct.
    struct Rectangle {
        width: u32,
        height: u32,
    }

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // Functions can also return structs.
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    let sq = square(10);
    println!("sq.width = {}", sq.width);
    println!("sq.height = {}", sq.height);

    // Tuple Structs
    // Tuple structs are similar to structs, but their fields have no names. They are useful when you want to give the whole tuple a name and make the tuple be a different type from other tuples, and naming each field as in a regular struct would be verbose or redundant.

    struct Color(u8, u8, u8);

    let red = Color(255, 0, 0);
    println!("red.0 = {}", red.0);

    // Function can also return tuple structs.
    fn black() -> Color {
        Color(0, 0, 0)
    }

    let black = black();
    println!("black.0 = {}", black.0);

    // Unit Structs
    // Unit structs are useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.
    struct Empty;

    impl Empty {
        // Implementing a trait on the Empty struct so that we can call the print method on it.
        fn print(&self) {
            println!("I'm unit struct!");
        }
    }

    let empty = Empty; // Creating an instance of the Empty struct
    empty.print(); // Calling the print method on the empty instance

    // Debugging with Structs
    // We can use the Debug trait to print out structs in a debug format.
    // {:?} is used to print out structs in a debug format.
    // #[derive(Debug)] is used to derive the Debug trait for a struct.
    #[derive(Debug)]
    struct Rectangle2 {
        width: u32,
        height: u32,
    }

    let rect2 = Rectangle2 {
        width: 10,
        height: 20,
    };

    println!("rect2 = {:?}", rect2); // Printing out the rect2 struct in a debug format

    // Implementing Methods for Structs
    // We can implement methods for structs. Methods are similar to functions, but they are defined within the context of a struct.
    struct Circle {
        radius: f64,
    }

    impl Circle {
        // Implementing a method called area for the Circle struct
        fn area(&self) -> f64 {
            3.14 * (self.radius * self.radius)
        }
    }

    let circle = Circle { radius: 2.0 };
    println!("circle.area() = {}", circle.area());
}
