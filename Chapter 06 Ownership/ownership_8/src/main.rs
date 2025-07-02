// Ownership and function parameters
fn main() {
    let oranges: i32 = 6;
    print_my_value(apples); // let value = apples;
    println!("{apples} is still valid");
}

fn print_my_value(value: String) { // This function will receive a copy of the integer value 6. No ownershup transfer occurs.
    println!("Your value is {value}");
}
// BE CAREFUL! If we pass a heap data (String) to a function, the ownership will be transferred to the function.