 fn even_or_odd(number: i32) {
    let result = if number % 2 == 0 { "even" } else { "odd" };
    println!("The number {number} is {result}");
 }

fn main() {
    even_or_odd(12);
    even_or_odd(5321);
}
