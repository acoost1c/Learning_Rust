fn intro() {
    let food: &str = "pasta"; // This type of string is done at compile time. It is forever like this in binary.
    let text: String = String::new();
    let candy: String = String::from("KitKat"); // Makes a new string from a string literal.
}

fn main() {
    let mut name: String = String::from("Nia"); // The Stack will reference the Heap
    println!("{name}");
    
    name.push_str(" Sedyakina"); // Length is longer now, capacity too. The Heap will allocate more space.
    println!("{name}");
}// Heap is magical, data can expand and contract. very cool