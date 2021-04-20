/////// Ownership
// Ownership is the main feature of Rust that makes it different than other programming languages.
// There are two major ways to deal with memory while programming:
// 1) Automated garbage collection - the language has an automated system in the background that
// consistently searches and frees up memory that's not being used while a program is running.
// 2) Manual - explicitly allocate and free memory on the system.

// Rust takes a third approach: memory is managed through a system of ownership.

/////// Stack and Heap
// Stack and heap are parts of memory available to your code at runtime.
// Stack stores LIFO; "pushing onto the stack" or "popping off of the stack".
// - All data stored on the stack must have a known, fixed size.
// Heap is a little more unorganized.
// - Storing data on the heap requires that you ask for a certain amount of space.
// - The memory allocator blocks out that space whether or not it will all be used.
// - It then returns a pointer (address to that location in memory).

// Allocating and dealing with data on the heap is slower all around.
// - It's further away.
// - It takes time to search for space.
// - It takes time to follow a point to the location to get the data.
// - It requires bookkeeping for future allocations.

// Ex. When your code calls a function, the values passed into the function
// (including, potentially, pointers to data on the heap) and the function’s
// local variables get pushed onto the stack. When the function is over,
// those values get popped off the stack.

// Ownership addresses the problems of keeping track of heap data,
// minimizing duplicate data and garbage collection.

// Ownership rules:
// - Each value in Rust has a variable called its owner.
// - There can only be one owner at a time.
// - When the owner goes out of scope, the value is dropped.

fn main() {
    { // s is not valid (in scope).
        let s = "hello"; // s is in scope.
    } // s is no longer valid, the scope is terminated.

    // In this case, a String type is allowed to be mutable, yet a literal (used in push_str()) cannot.
    let mut s = String::from("Hello"); // Syntax explained in Chapters 5/7
    s.push_str(", world!"); // push_str() appends to a string.
    println!("{}", s);

    // The Strategy:
    // Calling on String::from() begins the process of the memory allocation by asking for a certain space in memory.
    // In order for this to work conventionally, we have to correctly match up one allocation to one freeing of space.
    // Rust deals with this differently:
    // Memory is automatically returned once the variable that owns it goes out of scope.
    {
        let s = String::from("Hello"); // Memory is requested (allocated).
    } // Now out of scope, the 'drop' function is called and the memory is automatically returned to the allocator.

    // Simple when one variable has one value. What about when multiple variables access the same data (on the heap)?
    let x = 5;
    let y = x;
    // This is fine because integers are known, fixed sizes.

    // How about now?
    let s1 = String::from("Hello");
    let s2 = s1;
    // For a string, there are three pieces of info that remain on the stack (while the actual string is stored on the heap):
    // - A pointer (address to the location in memory where the actual string is stored).
    // - A length (amount of memory in bytes currently used by the string).
    // - A capacity (total amount given by the allocator).
    // A.k.a., both s1 and s2 both POINT to the SAME ITEM in memory.**
    // WARNING: when s1 and s2 go out of scope, they will both try to free up the same space in memory (double free error).

    // **To avoid the error mentioned above, when a value is 'copied' (like we tried above),
    // Rust invalidates the first variable. It 'moves' rather than 'copies' the data from s1 TO s2.
    // In that way, when these variables go out of scope, there is only one owner that will deallocate its memory.
    // Additionally, this means Rust will never automatically create deep copies.

    // If you do want deep copies:
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    // In this case, heap data will get copied.\

    // So what was happening with the integers then?
    // x is stored on the stack (since integers are fixed-size), so making copies is safe and efficient.
    // The Copy trait is also used on integers to allow an older variable to still be used after assignment.

    // Passing variables around to try to accomplish this can be tedious and that's when we use 'references'.
}
