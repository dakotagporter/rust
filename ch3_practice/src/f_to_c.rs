use std::io;

// Convert function taking a floating-point input (any continuous value may be requested to be converted by the user).
// Additionally returning a floating-point becuase the math involves division.
// Copies x's value to temp (which only uses space during this function)
fn convert(temp: f32) -> f32 {
    return (5.0/9.0)*(temp-32.0);
} // temp variable removed by 'drop'

fn main() {
    // Implement an infinite loop to make sure user inputs correct data type.
    loop {
        // User input prompt.
        println!("Type the number you'd like to convert to Celsius > ");

        // Create a mutable x variable to hold user's input. (requests memory on the heap due to unknown size of user input)
        // Optionally, declaring x before the loop will allow for use of the variable once the loop is finished.
        let mut x = String::new();
        // Read the user's input into the x variable.
        io::stdin()
            .read_line(&mut x)
            .expect("Failed to read line.");
        
        // Typecast the user input string into a float.
        let x: f32 = match x.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("\nConverting...\n");

        // Two options to manage the return value:
        // 1) Create new IMMUTABLE 'result' variable:
        //    - This allows x to still hold it's value and result will most likely NOT be changed later in the code.
        // 2) Overwrite x with the return value:
        //    - x is f32 and the return value of convert is the same, therefore this can be done.
        //    - This works when x only needs to be used once and can 
        let result = convert(x);
        // convert's return value is bound to result and printed below.
        // x holds its value and is still in scope and can also be printed below.
        println!("{} degrees Fahrenheit is {} degrees in Celsius.", x, result);
        break;
    } // Both x and result are terminated after leaving the loop.
}