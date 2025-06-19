// Rust does not care if we invoke a function BEFORE


fn main() {    /* here we put parametres in the invocation */
    open_store("Brooklyn");
    bake_pizza(6, "cheese");
    swim_in_profit();
    swim_in_profit();
    swim_in_profit();
    open_store("Queens");
    bake_pizza(3, "mushroom");
}

// Or after! (as long as its in the same scope, so Rust can see it)
                /* here we set arguments for the function */
fn open_store(neighborhood: &str) {
    println!("Opening my pizza store in {neighborhood}");
}

fn bake_pizza(number: i32, topping: &str) {
    println!("Baking {number} {topping} pizzas");
}

fn swim_in_profit() {
    println!("So much $$$, so little time");
}