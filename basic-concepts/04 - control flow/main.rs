fn main() {
    // let number = 3;
    // if number < 5 {
    //     println!("the condition is true");
    // } else {
    //     println!("the condition is false");
    // }

    // let number: i8 = 5;

    // if number != 0 {
    //     println!("number ain't zero");
    // }

    // let condition: bool = true;
    // let another_number = if condition { 5 } else { "six" };
    // println!("The value of another number is {another_number}");


    /*
        The expression in the if block evaluates to an integer, and the expression in the else block evaluates to a string.
        This won’t work because variables must have a single type, and Rust needs to know at compile time what type the number variable is, definitively.
        Knowing the type of number lets the compiler verify the type is valid everywhere we use number.
        Rust would not be able to do that if the type of number was only determined at runtime;
        the compiler would be more complex and would make fewer guarantees about the code if it had to keep track of multiple hypothetical types for any variable.
    */

    // let x = 1;
    // let y = if x { 0 } else { 1 };
    // println!("{y}");

    /*
        The condition to an if-expression must be a boolean.
        Rust does not have a concept of "truthy" or "falsy" values.
    */

    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("The result is {result}");

    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;
    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }
    //     count += 1;
    // }
    // println!("End count = {count}");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}