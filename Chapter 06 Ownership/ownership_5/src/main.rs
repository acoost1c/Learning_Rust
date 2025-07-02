// Clone method
fn main() {
    let person: String = String::from("Nia");
    let genius: String = person.clone(); // since we clone the string it will run fine! without stealing the ownership.

    println!("This is {}.", person);
    // Don't be afraid of using a little bit more memory, it's better than having a bug.
}
