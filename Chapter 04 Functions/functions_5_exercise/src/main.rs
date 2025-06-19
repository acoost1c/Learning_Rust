fn main() {
    apply_to_jobs(5, "Junior Maid");
    dbg!(is_even(6));
    println!("Is 6 an even number? {:?}", is_even(6));
    println!("Is 13 an even number? {:?}", is_even(13));
    println!("Does \"aardvark\" have an A,Z in it? {:?}", alphabets("aardvark"));
    println!("Does \"zebra\" have an A,Z in it? {:?}", alphabets("zebra"));
    println!("Does \"irony\" have an A,Z in it? {:?}", alphabets("irony"));
}

fn apply_to_jobs(number: i32, title: &str) {
    println!("I'm applying to {number} {title} jobs.")
}

fn is_even(number: i32) -> bool {
    number % 2 == 0
}

fn alphabets(text: &str) -> (bool, bool) {
    let a = text.contains('a');
    let z = text.contains('z');
    return (a, z)
    /*
    (text.contains('a'), text.contains('z'))
     */
}