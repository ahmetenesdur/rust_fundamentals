// Enums

fn main() {
    // Enums is a type that can have a few definite values (variants)
    enum Weather {
        Sunny,
        Cloudy,
        Rainy,
        Snowy,
    }

    let today = Weather::Sunny; // today is of type Weather enum and its value is Sunny

    // Enums with Associated Data
    // Enums can also have data associated with each variant (like a struct)
    enum WeatherWithTemp {
        Sunny(u8),
        Cloudy(u8),
        Rainy(u8),
        Snowy(String),
    }

    let tomorrow = WeatherWithTemp::Sunny(32); // tomorrow is of type WeatherWithTemp enum and its value is Sunny with 32 degrees
    let yesterday = WeatherWithTemp::Snowy(String::from("minus ten")); // yesterday is of type WeatherWithTemp enum and its value is Snowy with minus ten degrees

    fn weather_report(weather: WeatherWithTemp) {
        match weather {
            WeatherWithTemp::Sunny(temp) => println!("It's sunny with {} degrees", temp),
            WeatherWithTemp::Cloudy(temp) => println!("It's cloudy with {} degrees", temp),
            WeatherWithTemp::Rainy(temp) => println!("It's rainy with {} degrees", temp),
            WeatherWithTemp::Snowy(temp) => println!("It's snowy with {} degrees", temp),
        }
    }

    weather_report(tomorrow);

    // The ‘if let’ Syntax
    // if let is a syntax that allows you to combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest
    enum Animal {
        Car(String),
        Dog(String),
        Bird(String),
    }

    let my_pet = Animal::Dog("Rusty".to_string()); // .to_string() is a method that converts a string slice (&str) to a String

    if let Animal::Dog(name) = my_pet {
        println!("I have a dog named {}", name);
    } else {
        println!("I don't have a dog");
    }

    // Enums and Methods in Rust
    // Enums can have methods just like structs
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            match self {
                Message::Quit => println!("Quit"),
                Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
                Message::Write(text) => println!("Write {}", text),
                Message::ChangeColor(r, g, b) => {
                    println!("Change color to r: {}, g: {}, b: {}", r, g, b)
                }
            }
        }
    }

    let msg = Message::Move { x: 10, y: 20 };
    msg.call();
}
