fn main() {
    let mut current_meal = String::new();
    current_meal = add_flour(current_meal);
    add_sugar();
    add_salt();
}

fn add_flour(mut meal: String) -> String {
    meal.push_str("Add flour");
    meal
}

fn add_sugar() {}

/*
    This is not scalable and very cumbersome to constanlty return the value.
    Better way will be learned soon.
 */