// Ownership in Rust
// Ownership is a set of rules that the compiler checks for to ensure the program will be free of memory errors.
// Owner is what is responsible for cleaning up a piece of data when its no longer in use.
// Every value in a Rust program has one owner.
// The owner can change over the course of the program, but the is only 1 owner for a value at a time.
// The owner is usually a name.
// Ownership also extends to composite types that own their elements. (A tuple and array own their values)

fn main() {
    let age: i32 = 33; // age is the owner of the value 33
    let is_handsome: bool = true;
    
    println!("{age}");
    println!("{is_handsome}"); // left on the stop of the stack

    // age variable still exists here
} // When the variable goes out of scope, its the owner that deallocates the memory. The owner will then popout the data from the stack.
// is_handsome goes out of scope, then age variable goes out of scope. Then the program is done. LIFO (Last in, first out).
