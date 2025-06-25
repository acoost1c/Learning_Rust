fn main() {
    let season = "summer";

    // if season == "summer" {
    //     println!("School's out!");
    // } else if season == "winter" {
    //     println!("Brr, so cold!");
    // } else {
    //     println!("Lots of rain!");
    // }

/* Practically does the exact same thing! Seeks the first match */

    match season {
        "summer" => println!("School's out!"),
        "winter" => println!("Brr, so cold!"),
        _ => println!("Lots of rain!"), // Do not put this first, it will always match and the rest will be ignored!
    }
}
