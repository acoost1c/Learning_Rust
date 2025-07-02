// Copy trait

fn main() {
    let time: i32 = 2025;
    let year: i32 = time; // taking up twice the memory since there is two owners.

    println!("The time is {time}. It is the year {year}.");
}
// In stack memory, the data is copied.