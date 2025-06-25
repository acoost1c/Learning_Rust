// Iteration
fn main() {
    whileloop();
    let mut seconds = 21;

    loop {
        if seconds <= 0 {
            println!("Blastoff!");
            break;
        } // we put the if statement so the blastoff happens at 0 seconds

        if seconds % 2 == 0 {
            println!("{seconds} seconds (even number), skipping 3 seconds..");
            seconds -= 3;
            continue;
        }
        // we put the continue statement so the loop skips the println below if the seconds are even

        println!("{seconds} seconds to blastoff...");
        seconds -= 1; // shortcut for seconds = seconds - 1
    }
    // always break a loop with a break statement
}

// while loop example
fn whileloop() {
    let mut secondsw = 13;

    while secondsw > 0 {
        // if seconds <= 0 {
        //     println!("Blastoff!");
        //     break;
        // } we dont need that here because the while loop will stop when seconds is 0

        if secondsw % 2 == 0 {
            println!("{secondsw} seconds (even number), skipping 3 seconds..");
            secondsw -= 3;
            continue;
        }

        println!("{secondsw} seconds to blastoff...");
        secondsw -= 1;
    }
    println!("Blastoff!");
    // we put the blastoff here because the while loop will stop when seconds is 0
}