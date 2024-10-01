use std::io::{self, Write, Read};
use std::fs::File;

struct Car {
    color: String,
}

fn reading_from_console_car() {

    // Takes in user input
    println!("What color is your car? ");
    let mut buffer = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let color = buffer.trim().to_string();
    buffer.clear();

    // Assigns car struct with what user inputted
    let car = Car { color };

    // Put this in a newly created file
    let mut file = File::create("cars.txt").unwrap();
    writeln!(file, "The color of your car is {}!", car.color).unwrap();
    let mut file = File::open("cars.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("File contents:\n{}", contents);
}

fn main() {
    reading_from_console_car();
}
