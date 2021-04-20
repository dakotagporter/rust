////// Functions

// main is the entry point of a rust program.
fn main() {
    println!("Hello, world!");
    another_function(5, 6);
}

// Functions can have parameters.
fn another_function(x: i32, y:i32) {
    println!("The value of x is {} and the value of y is {}.", x, y);
}

// Rust is an expression-based language.
// Statements do something (end with semicolons), expressions return a value (do NOT end with semicolons).
// Statement:
let y = 6;
// Expression:
y + 1

// Return values:
// Declare a function's return value:
fn five() -> i32 {
    return 5;
}

let x = five();