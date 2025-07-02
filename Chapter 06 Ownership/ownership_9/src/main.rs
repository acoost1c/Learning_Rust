// mutable parametres
fn main() {
    let mut burger = String::from("Burger");
    //let meal:String = burger; same thing
    //meal.push_str( " and Milkshake "); we can't push string on the immutable variable no matter what!
    add_fries(burger); // let meal = burger same thing
}

fn add_fries(mut meal: String) { // ownership moves to meal parametre WHICH IS IMMUTABLE RIGHT now, but I made it mutable just now
    meal.push_str(" and Fries ");
    println!("{meal}");
}