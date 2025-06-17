fn main() {
    let mut gym_reps = 10; // "mut" means mutable, to be able to change. We are changing the variable so do not forget the "mut".
    println!("I plan to do {gym_reps} reps");

    gym_reps = 15;
    println!("I now plan to do {gym_reps} reps");
}
