// Unit - an empty tuple, a tuple without values.
fn main() {
    /* let result = (5, true, "hello"); regular tuple */
    let result () = (); // unit
    let result () = mystery(); // Rust is smart so if a funtion is nothing (cuz nobody knows) it makes a guess its a unit
}
        /* no need to specify */
fn mystery() /*-> ()*/ {
    println!("Helloo!!!"); // even if the funtion does something, it does not have a return value, hence it will be a unit!
}