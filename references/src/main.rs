fn main() {
    let s1 = String::from("Hello");
    let len = caluculate_len(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("Hello");
    change(&mut s); // The reference must be declared mutable just like a variable if you want to change it's value.

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2); // The immutable references now exit scope.

    let r3 = &mut s; // Mutable reference can now be introduced.

    let reference_to_nothing = dangle(); // Causes 'lifetime' compiler error (covered in Chapter 10)
}


fn caluculate_len(s: &String) -> usize { // s is a reference to a string
    s.len()
} // s goes out of scope, but because s does not have ownership of what it refers to, nothing happens.

fn change(some_string: &mut String) { // String type HAS to be mutable in order to change the borrowed value.
    some_string.push_str(", world!");
}

fn dangle() -> &String { // returns a reference to a string
    let s = String::from("Hello"); // s is a new string
    &s // return reference to string s
} // s goes out of scope and the memory is thrown away. Instead, return the string directly (below)

dn no_dangle() -> String {
    let s = String::from("Hello");
    s
}

////// References and Borriwing
// & is a reference. &s1 refers to but does NOT own the string.
// s points to s1 that points to the heap data of the string.
// The scope of a mutable reference lasts until the last time the reference is called. (example above)

// Borrowing: having references as function parameters. (Similar to in-place operations)
//   - A function can borrow a value from an owner (like people would).
//   - It can use it, but must return it. It CANNOT be changed. (Unless declared mutable)
//   - You can only have ONE MUTABLE reference to a piece of data in a particular scope.
//      - Also cannot have a mutable and immutable reference at the same time.
//      - A new reference can be created in a new scope (meaning it won't be simulatneous).
//      - This restriction prevents data races:
//          - Two or more pointers access the same data at the same time.
//          - One of the pointers is being used to write to the data.
//          - There's no mechanism being used to synchronize access to the data.

/////// Dangling References
// These references point to something empty or space that was given to something else.
// In Rust, it is guaranteed that no references will dangle.
//  - If you have a reference, the compiler ensures that the data will not go out of scope before the reference does.