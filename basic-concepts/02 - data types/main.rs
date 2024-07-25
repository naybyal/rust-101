use std::io;

fn main() {
    let angry_face = '😠';
    println!("{}", angry_face);

    // tuple 
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // general way of grouping together a number of values with a variety of types into one compound type
    // let (_x, _y, _z) = tup;
    // println!("The value of y is: {}", _y);
    // unused variables should be prefixed with an underscore
    
    // let five_hundred = tup.0;
    // println!("The value of five_hundred is: {}", five_hundred);


    // array
    // let a = [1, 2, 3, 4, 5];
    // arrays in Rust have a fixed length
    // let first = a[0];
    // let second = a[1];
    // println!("The value of first is: {}", first);
    // println!("The value of second is: {}", second);
    // accessing an element of an array using indexing
    // Rust is zero-indexed
    // accessing an element of an array that is out of bounds will result in a runtime error
    // Rust will not compile the program if it contains this error
    // Rust checks that the index is less than the array length
    // Rust checks that the index is greater than or equal to zero
    // Rust checks that the index is an integer
    // Rust checks that the array has the type that the index specifies
    // Rust checks that the index is a constant
    // Rust checks that the index is not a variable or an expression that can change at runtime
    // Rust checks that the index is not a negative number
    // Rust checks that the index is not a number greater than or equal to the array length
    // Rust checks that the index is not a number greater than or equal to the maximum value of the integer type
    // Rust checks that the index is not a number greater than or equal to the maximum value of the usize type
    // Rust checks that the index is not a number greater than or equal to the maximum value of the isize type
    // Rust checks that the index is not a number greater than or equal to the maximum value of the u8 type

    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    // arrays in Rust have a fixed length
    // the type of the array is [i32; 5]
    // the type of the array is an array of 32-bit signed integers with a length of 5

    // let a = [3; 5];
    // this initializes the array to contain the same value for each element by specifying the value, followed by a semicolon
    // followed by a semicolon, and then the length of the array in square brackets 



    // invalid array element access
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}