fn main() {
    let is_concert: bool = true;
    let is_event = is_concert;
    // I think the owner is going to be is_concert
    
    println!("1: {}\n2: {}", is_concert, is_event); // I was right, or not???? No i wasn't ownership moved.

    let sushi: &str = "Salmon";
    let dinner = sushi;
    // No the sushi is a String literal and is inside the binary code

    println!("{sushi}, {dinner}"); // I was right here though

    let sushi_heap: String = String::from("Salmon");
    println!("Imma eat {sushi_heap} sushi");
    //let dinner_heap = sushi_heap;
    //println!("{}, {}", sushi_heap, dinner_heap); here it can't print because ownership was moved

    let sushi_heap = eat_meal(sushi_heap);
    println!("Imma eat {sushi_heap:?} sushi",);
}

fn eat_meal(mut meal: String) -> String {
    meal.clear();
    meal
}
