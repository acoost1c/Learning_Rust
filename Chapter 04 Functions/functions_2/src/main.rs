// Return values
fn main() {
    let result = square(5);
    println!("The square of 5 is {result}.");

    let result = square(13);
    println!("The square of 13 is {result}.");
}
/* specifying the type of the return value with an arrow -> */
fn square(number: i32) -> i32 {
    return number * number; // no code lives after the return! its dead after that 🥀
}
fn square_implied(number: i32) -> i32 {
    number * number // since its the last line Rust automatically knows that its the implied return value 
}