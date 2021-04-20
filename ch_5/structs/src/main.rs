fn main() {

    // Struct defining a user.
    struct User {
        username: String, // Use the owned String type rather than &str so that instances own ALL of their data.
        email: String, // If a field stores a reference, it's important to pay attention to lifetimes (that will ensure the data lasts throughout the life of the struct).
        sign_in_count: u64,
        active: bool
    }

    // Create an instance of the struct:
    let mut user1 = User {
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        active: true,
        sign_in_count: 23
    };

    // Change user1's email value.
    user1.email = String::from("anotheremail@example.com");

    // Struct update syntax.
    let user2 = User {
        email: String::from("sample@email.com"),
        username: String::from("anotherusername456"),
        ..user1 // The remaining fields not set for user2 (active, sign_in_count) should be filled in based on user1's info.
    };

    // Tuple Struct
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);



    // EXAMPLE PROGRAM

    //Simple
    let width = 30;
    let height = 50;
    // area(width, height)

    //Tuples
    let rect1 = (30, 50);
    // area(rect1)

    //Structs
    #[derive(Debug)] // Explained below.
    struct Rectangle {
        width: u32,
        height: u32
    }
    let rect1 = Rectangle {
        width: 30,
        height: 50
    }
    // area(&rect1) -> This way 'main' can continue to own rect1 and use it in the future.

    println!("The are of the rectangle is {} square pixels.", area(&rect1));

}

// Wrap struct in a function to generalize the process.
fn build_user(email: String, username: String) -> User {
    User {  // Using init shorthand, we can avoid typing the field: variable names multiple times.
        username, // Can be used since the field name and variable name are the same.
        email,
        sign_in_count: 1,
        active: true
    }
}

// EXAMPLE PROGRAM
fn area_simple(width: u32, height: u32) -> u32 { // The two parameters are related, but that's not specified in the code...
    width * height
}

// Refactor w/ tuples
fn area_tuples(dimensions: (width, height)) -> u32 { // Tuples don't name their arguments...
    dimensions.0 * dimensions.1
}

// Refactor w/ structs
fn area(rectangle: &Rectangle) -> u32 { // Parameter is an "immutable borrow of a struct Rectangle instance"
    rectangle.width * rectangle.height
}

/////// Structs
// Structs help you name and package multiple related values that make up a meaningful group.
// Similar to an objects attributes in object-oriented programming.


//// Defining and Instantiating
// Similar to tuples, structs can contain different types.
// Similar to a dictionary/class object. Explicitly name each fields type.
// Use dot notation to receive values. ENTIRE struct has to be mutable.
// Struct update syntax allows you to use an old instances info to 'update' a new instance.

//// Tuple Structs
// Some structs can be defined to look like tuples. These allow you to just define a quantity of data types (no field names).
// The instantiations of tuple structs can be treated like a tuple as well (accessing, unpacking)

// Printing Instances:
//  - It'd be nice to print a struct instance (for debugging, etc.)
//  - println!("{}", instance) won't work due to std::fmt::Display errors when formatting.
//  - The error suggests this solution: println!("{:?}", instance). ":? or :#? (for larger structs)" - Debug format...
//  - Still get an error saying the Debug "trait" is not implemented in your struct:
//  - Providing #[derive(Debug)] above the struct allows the struct to access the Debug trait. (__str__ Python equivalent)

//// Methods
// Methods allow us to tie functions to a particular struct (like the Rectangle from above). Defined within the context of a struct.
// Helpful to understand and organize functionality of objects and data in your program.
// ***First parameter is ALWAYS self; represents the instance of the struct that the method is beign called on.
fn refactored_area() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32
    }

    impl Rectangle { // Implementation block
        fn area(&self) -> u32 { // Use & here for the same reason as &Rectangle from above. We only want to "read not write".
            self.width * self.height // Self (the instance being processed) has the attributes width and height therefore we can access them using dot notation.
        }
    }
}
// Use method syntax to call on the function: rect1.area()