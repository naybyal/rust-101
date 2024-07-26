use std::io;

fn fibonacci(n: u32) -> u32 {
    if n == 1 { 1 }
    else if n == 0 { 0 }
    else {
        (fibonacci((n-1) as u32) + fibonacci((n-2) as u32)) as u32
    }
}

fn main() {
    println!("Nth Fibonacci Number\nReading n :");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read n");

    let n: usize = n
        .trim()
        .parse()
        .expect("Failed to parse n to integer");

    let answer = fibonacci(n as u32);

    println!("The Nth Fibonacci Number is {answer}");
}