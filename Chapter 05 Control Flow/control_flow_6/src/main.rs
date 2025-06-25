fn main() {
    let number = 100;

    match number {
        value if value % 2 == 0 => println!("{value} is an even number"), // can use any name
        x if x % 2 != 0 => println!("{x} is an odd number"),
        _ => unreachable!(), // unreachable!() is a macro that lets the compiler know that this code path should never be reached.
    }
}
