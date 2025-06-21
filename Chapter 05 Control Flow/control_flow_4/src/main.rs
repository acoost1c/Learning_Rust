// match statement - enables us to react to multiple variants of a value
fn main() {
    let evaluation = false;
    
    match evaluation {
        // pattern or an arm is one possible option to compare the match value against
        true => {
            println!("The value is true.");
        }
        false => {
            println!("The value is false.");
        }
    };
    let value = match evaluation {
        true => 20, //SO IF THE EVALUATION IS TRUE, THE VALUE WILL BE 20
        false => 40, // BUT IF THE EVALUATION IS FALSE, THE VALUE WILL BE 40
    };

    println!("{value}"); // so now, if the evaluation is true, it will print 20, and if the evaluation is false, it will print 40
    // this is a better way of doing it than using else if statements
}
