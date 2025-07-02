fn main() {
    let person: String = String::from("Nia"); // becomes invalid if the commented section is active
    println!("My name is {}.", person); // works!

    //let genius = person; // genius becomes the owner of the String
    //println!("My name is {}", person); DOES NOT WORK! person is no longer valid!!! genius is now the owner.

    /* Drop function */
    drop(person); // manually drop the variable from memory
    println!("My name is {}", person); // DOES NOT WORK! person is no longer valid!!! genius is now the owner.
    //let genius: String = person; invalid as well
}
