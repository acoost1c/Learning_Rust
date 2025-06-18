// Ranges and range iteration
fn main() {
    let month_days = 1..31; // Exclusive range, it goes from 1 to 30, 31 is excluded
    println!("{month_days:?}");
    let month_days = 1..=31; // Inclusive range, goes from 1 to 31. Nobody left behind.
    println!("{month_days:?}");

    /* Iteration from any given number to the  */
    for number in month_days {
        println!("{number}");
    }

    let letters = 'b'..'f'; // Works with letters!
    for letter in letters {
        println!("{letter}");
    }

    let colors = ["Red", "Green", "Yellow"];
    for color in colors {
        println!("{color}");
    }

    let month_days: std::ops::Range<i32> = 1..31;
    let letters: std::ops::Range<char> = 'b'..'f';
    // Value - 5, Type - i32
}
