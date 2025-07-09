// Dangling reference - is a pointer to a memory address that has been deallocated.
fn main() {
    let city: String = create_city();
    println!("{city}");
}

fn create_city() -> String {
    String::from("New York")
}
