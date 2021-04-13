// cargo doc --open will open all documentation (both Rust and dependencies) in your browser.

// Use the standard input/output library
use std::io;
// "Library Crate" rand from Crates.io (PyPI equivalent) added to Cargo.toml file.
// Ran cargo build again to install the dependencies
// Using Rng (random number generator) trait
use rand::Rng;
// Compare: Like Result, Ordering is an 'enum' with variants Less, Greater, Equal
use std::cmp::Ordering;

// Define a function w/ no parameters.
fn main() {
    // Macro printing a string to the screen.
    println!("Guess the number!");

    // thread_rng() defines the specific rng that will be used: local to current thread and seeded by OS.
    // gen_range() takes range expression (start..end), alternatively: gen_range(1..=100)
    let secret_number = rand::thread_rng().gen_range(1..101);

    // Infinite loop.
    loop {
        println!("Please input your guess.");

        // Create a variable to store user input.
        // let is used to create a variable. ex. `let foo = bar;`
        // Variables are immutable by default in Rust, mut allows variable to be mutable.
        let mut guess = String::new();
        // String is a string type provided by the standard library that is growable, utf-8 encoded.
        // :: means that the new function is an "associated function" of the String type.
        // A.k.a. a static method, an associated function is implemented on a type (class?) rather than an instance.
        // String::new() creates a new, empty instance of a string.
    
        // Returns an instance of Stdin from the io module. Alternatively std::io::stdin()
        io::stdin()
            // Calls readline method on the Stdin object
            // Overrides guess (an empty string) with whatever the user input.
            // & is a reference to allow multiple parts of your code access one piece
            // of data without having to copy it into memory multiple times.
            .read_line(&mut guess)
            // read_line assigns the user input to the string but also returns an io::Result type that can be 'Ok' or 'Err'.
            // These are called variants. Within the variant is either the generated result or the error, respectively.
            // Leaving out the expect function would still allow the program to compile but would bring up a
            // warning letting you know that this process may result in an error and that it should be taken into account.
            .expect("Failed to read line.");
            // Could have been written as: io::stdin().read_line(&mut guess).expect("Failed to read line.");
        
        // "Shadows" (typecasts) the guess variable.
        // : tells Rust that we will define the variable type.
        // u32: Unsigned 32-bit integer is a good default for small integers. Rust infers that secret_number will be u32 as well.
        // .trim(): Python .strip() equivalent; trims whitespace and newlines.
        // .parse(): Parses the string into a number (based on variable type defined).
        // Because the .parse() might fail if the variable cannot be shadowed, the expect catches the error.
        // Allow user to continue guessing by customizing how we handle the error:
        // Use a match expression to assess the variantes and execute the subsequent code if triggered.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
    
        // Prints the variable that we saved the user input in.
        // Similar to Python format strings.
        println!("You guessed: {}", guess);
    
        // match expression will evaluate the comparison and run one of its "arms" based on the variant returned.
        // An "arm" consists of a "pattern" (Ordering::*) and the code (println!) to run depending on the match.
        // Warning: guess is a string and secret_number is a 32-bit integer (default) and, therefore, cannot be compared.
        // Fix this by adding line {}
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
