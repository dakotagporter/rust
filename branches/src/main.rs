fn main() {
    let number = 3;

    // If statements are expressions and do NOT end in semicolons.
    // Conditions must be bools.
    if number < 5 {
        println!("Condition was true!");
    } else if number > 5 {
        println!("Condition was false!");
    } else {
        println!("The number is 5!");
    }

    // If statements can be assigned to a variable with let.
    // If and Else arms MUST have the dame data types.
    let condition = true;
    let number = if condition {5} else {6};
}
