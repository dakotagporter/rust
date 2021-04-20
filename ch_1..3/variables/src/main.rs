fn main() {
    ////////// Variables and Mutability

    // Rust encourages embracing immutability as all variable are set to be immutable by default.
    // The following code gives an error because x is originally assigned immutable and cannot change.
    // This helps by preventing bugs where a value is only changed sometimes.
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Adding the mut keyword allows a variable to become mutable.
    let mut x = 5;
    // -- snip --

    // Immutable variables are similar to constants. Use const instead of let.
    // Constants, however, don't allow you to use the mut keyword.
    // Type of variable MUST be annotated. Underscores and capitalization based on Rust convention.
    const MAX_POINTS: u32 = 100_000;

    // Shadowing allows declaration of a new variable with the same name as
    // another variable by reusing the let keyword.
    // Shadowing also keeps the orginal variable immute and can perform these
    // actions without error and the new value will again be immutable.
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    // output = 12

    // The variable type can be changed as well because we are
    // effectively creating a new variable each time we use let.
    // In any of these cases, compile-time errors come up if the variable is mutable.
    let spaces = "    ";
    let spaces = spaces.len();


    ////////// Data Types

    // It needs to know the types of all variables when compiling.
    // When there are many types to choose from, annotation is required (i.e., numeric types).
    let guess: u32 = "42".parse().expect("Not a number!");

    // Two subsets: scalars and compounds. Rust is a statically-typed language.

    // Scalar: single value (ints, floats, bools, and chars)
    // - Ints: signed(i)/unsigned(u) (can be negative/only ever positive) taking up to (16, 32 (default i32), 64, 128, size) bits of space. Uses two's complement.
    //   - Integer overflow can occur: use std wrapping_*, checked_* and overflowing_* methods to handle.
    // - Floats: f32, f64 (default) single v. double precision.
    // - Booleans: one byte in size, use bool keyword when annotating.
    // - Chars: specified in single-quotes, four bytes in size (meaning a TON of characters)

    // Compounds: multiple values in one type (tuples and arrays)
    // - Tuples: fixed length, contain different types in parenthesis, annotate the type of each member if desired.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //   - Unpack ("destructure") to get individual members or use dot notation with the desired index:
    let (x, y, z) = tup;
    let val = tup.2
    // - Array: must be the same type in square brackets.
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // Assigning array similar to annotation; result = [3, 3, 3, 3, 3]
    let a = [3; 5];
    //   - Useful for data on the stack not the heap or when ensuring a fixed number of elements.
    //   - A vector, however, can grow or shrink.
    //   - Accessing elements is just like Python lists
}
