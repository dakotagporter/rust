fn main() {
    let mut s = String::from("Hello, world!")

    let hello = &s[..5]; // Hello
    let world = &s[7..13]; // world

    let word = rewrite_first_word(&s); // word is an immutable reference to a slice of s (according to the return value).

    s.clear(); // The clear function needs a mutable reference to s, but since there is already an IMmutable reference, an error occurs.
    
}

// Function that returns the first word of a string (or the whole string if it doesn't contain spaces).
fn first_word(s: &String) -> usize { // Compiles but is dangerous because the string can change after the first_word function is called.
    let bytes = s.as_bytes(); // Convert string to array of bytes.

    for (i, &item) in bytes.iter().enumerate() { // Iterators discussed in Chapter 13. Enumerate returns a tuple of an index and the reference to the item at that index.
        if item == b' ' { // Use byte literal to find spaces.
            return i
        }
    }

    s.len() // Return the length of the string if no spaces are found.
}

fn rewrite_first_word(s: &str) -> &str { // str refers to a string slice type.
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }

    &s[..]
}

/////// Slices
// Similar to slices in Python, slices in Rust do NOT have ownership.
// String literals:
let s = "Hello, world!"
//  - Here, s is an immutable because it's type is &str (a pointer to a specific location in binary)
//  - You can take slices of both literals and String types.
//  - Using rewrite_first_word function above is the most generalized we can make it. It will take
//    both String and &str types.
//  - Can also slice arrays.

////// All the process ensure memory safety at compile time.
// The Rust language gives you control over your memory usage in the same way as other systems programming languages, but having the owner of data automatically
// clean up that data when the owner goes out of scope means you don’t have to write and debug extra code to get this control.