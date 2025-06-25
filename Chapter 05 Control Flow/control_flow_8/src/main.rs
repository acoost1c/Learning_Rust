// Recursion - is when a function calls itself

fn countdown(seconds: i32) {
    if seconds == 0 {
        println!("Blastoff!");
    } else {
            println!("{seconds} seconds to blastoff..");
        countdown(seconds - 1);
    }
}

fn main() {
    // base case is the condition that stops the recursion
    countdown(5);
}
