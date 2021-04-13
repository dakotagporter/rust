// Main function contains the first code that is executed. rustfmt is the style formatting.
fn main () {
    // Four space indent
    // ! calls a "Rust Macro" instead of a normal function
    // ; means end of expression
    println!("Hello, world!");
}

// Compile by running `rustc <filename>.rs` (creates binary executable) then `./<filename>`
// "ahead-of-time compiled" languages (Rust, C++, etc.) mean you can send a compiled program
// to someone and they can run it without even having the compiler installed.