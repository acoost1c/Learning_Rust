// Borrowing
fn main() {
    let my_stack_value: i32 = 2;
    let my_integer_reference: &i32 = &my_stack_value; // & is for borrowing, think of it as an address that leads to the integer!
    println!("{}", *my_integer_reference); // derefencing

    let my_heap_value = String::from("Toyota");
    let my_heap_reference =&my_heap_value;

    println!("{}", *my_heap_reference); // dereferencing

    /* Without dereferencing we print the original values */
    println!("Integer reference: {}, Heap reference:  {}.", my_integer_reference, my_heap_reference); // compiler will guarantee that the reference is valid, so no need to check for null

    // Dereference operator. To dereference means to access the value that a reference points to.
}
