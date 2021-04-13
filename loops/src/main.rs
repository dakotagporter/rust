fn main() {

    // Infinite loop until explicit termination.
    loop {
        println!("Again!");
    }

    // Advantages of loop:
    // Allows you to retry an operation that might fail.
    // Any values that want to be return after the loop's termination
    // can be returned in the break statement.
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result was {}.", result);

    // Conditional loops with while:
    // Evaluates a condition through each loop iteration.
    // Runs if true and terminates if false.
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("Liftoff!!");

    // Looping through a collection with for:
    // While loops can be error prone when searching through collections.
    let a = [1, 2, 3, 4, 5];

    for element in a.iter() {
        println!("The value is: {}.", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("Liftoff!!")
}
