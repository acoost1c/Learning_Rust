// arrays time

fn main() {
    let numbers: [i32; 6] = [4, 8, 15, 16, 23, 42]; // arrays are fixed-size collection of homogenous data

    let apples = ["Granny Smith", "Macintosh", "Red Delicious"]; //any type is fine
    println!("Length: {}.", apples.len()); // length of an array operator.

    let currency_rates: [f64; 0] = []; // the array can be empty too!

    // reading and writing the array elements.
    let mut seasons: [&str; 4] = ["Spring", "Summer", "Fall", "Winter"]; //count them from 0! always, it's a computer not a humanutor
    let first =seasons[0];
    let second = seasons[1];
    println!("The {first} is the first season. The {second} is second.");

    // println!("{}", seasons[100]); Won't compile because there is no such element in the array!
    println!("{}", seasons[2]);
    seasons[2] = "Autumn";
    println!("{}", seasons[2]);
}
