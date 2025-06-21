// else if
fn main() {
    let season = "fall";

    if season == "summer" {
        println!("School is out 💖✨")
    } else if season == "winter" {
        println!("School is still going, very cold 🥶❄️")
    } else if season == "fall" {
        println!("Leaves falling 🍂")
    } else if season == "spring" {
        println!("It's raining a lot 🌈🌧️🌦️")
    } else {
        println!("Woooow!!!") // could be used to cull same outcomes
    };

    if season == "summer" {}; // Not advised!
    if season == "winter" {}; // Rust will have to check every single time, while using "else if" is more efficient and will stop right after a match!

    let some_conditional = true;
    if some_conditional {
        //whatever
    } else {
        //whatever
    }
}
