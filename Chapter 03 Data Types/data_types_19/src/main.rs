// Exercise time
fn main() {
    let l33t: i32 = 1_3_3_7;

    let leet = l33t as i16;

    let fl33t: f64 = 1.33700420;
    println!("{fl33t:.3}");

    let with_milk: bool = true;
    let with_sugar: bool = false;
    let is_my_type_of_coffee: bool = !(with_milk || with_sugar);
    let is_acceptable_coffee: bool = !with_sugar;
    println!("Coffee has milk, {with_milk}. Has sugar, {with_sugar}. I like it, {is_my_type_of_coffee}. It is acceptable, {is_acceptable_coffee}.");

    let places: [i8; 3] = [4, 2, 1];
    println!("{places:?}");
    let tuple = (16, 9.999, false, places);
    dbg!(tuple);
}
