fn main() {
    let apples = 50;
    let oranges = 14 + 6;
    let _fruits = apples + oranges;

    /* Could be written like so for readabilty and ease of use:
                :3 big W
    println!("This year, my garden has {apples} apples and {oranges} oranges.");
    */

    println!(
        "This year, my garden has {0} apples and {1} oranges. Ain't no way I got {0} apples!", // we can write numbers to not repeat the same argument!
        apples,
        oranges // Count from 0!!! apples -> 0, oranges -> 1.
    );
}
