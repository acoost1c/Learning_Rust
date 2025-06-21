// If statement
fn main() {
    let some_condition_that_we_cannot_predict_in_advance = true;
    if some_condition_that_we_cannot_predict_in_advance { //Has to be a boolean!!!
        println!("This line will be output")
    }

    if some_condition_that_we_cannot_predict_in_advance { //Has to be a boolean!!!
        println!("This line will not be output")
    }
}
