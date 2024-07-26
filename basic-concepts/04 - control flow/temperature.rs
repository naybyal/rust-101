use std::io;
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    ((9.0/5.0) * celsius) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * (5.0/9.0)
}
fn main() {
    println!("--- Temperature Converter ---\n\n0 for F-to-C, 1 for C-to-F\nReading the choice : ");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read choice.");

    let choice: usize = choice
        .trim()
        .parse()
        .expect("Failed to parse choice to integer");

    let mut magnitude = String::new();
    println!("Reading the magnitude : ");
    io::stdin()
        .read_line(&mut magnitude)
        .expect("Failed to read the magnitude");

    let magnitude: isize = magnitude
        .trim()
        .parse()
        .expect("Failed to parse magnitude to integer");

    if choice == 0 {
        let answer: f64 = fahrenheit_to_celsius(magnitude as f64);
        println!("It is {:.2}degC!", answer);
    } else if choice == 1 {
        let answer: f64 = celsius_to_fahrenheit(magnitude as f64);
        println!("It is {:.2}degF!", answer);
    }
}