fn main() {
    println!("Hello, world!");
}

// Running `cargo new hello_cargo` generates
// a new directory ("crate") hello_cargo with the following (similar to a Python package):
// - .gitignore file
// - Cargo.toml (Tom's obviously minimal language) (aka Pipfile)
// - src directory (contains the actual code)
//    - main.rs with an example Hello, world! program.

// Running cargo build from the top level,
// creates a Cargo.lock (Pipfile.lock) and
// a target directory where the executable can be
// ran with `./target/debug/<crate_name>

// `cargo run` will compile and execute the code in one line

// `cargo check` check if it compiles without error but
// does NOT create the executable.

// `cargo build --release` builds a final optimized executable
// in ./target/release for production.