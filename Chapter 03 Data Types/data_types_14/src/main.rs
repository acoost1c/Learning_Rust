fn main() {
    let first_initial: char = 'N'; //Character type! One and only.
    let emoji: char = '😎';
    println!("{}, {}.",
    first_initial.is_alphabetic(),
    emoji.is_alphabetic());

    println!("{}, {}.",
    first_initial.is_uppercase(),
    emoji.is_lowercase());
}
