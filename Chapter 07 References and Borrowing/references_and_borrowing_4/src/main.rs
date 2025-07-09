fn main() {
    let coffee: String = String::from("Mocha");
    let a = &coffee;
    // let b = a; its the same as below! COPY TRAIT!
    let b = &coffee;
    println!("{a} and {b}");
}

fn main_mut() {
    let mut coffee: String = String::from("Mocha");
    let a = &coffee;
    let b = a; // its the same as below! COPY TRAIT!

    println!("{a} and {b}");
} // Ownership has been moved to b, which is an error for the compiler since we try to use the same memory by two references