fn main() {
    let multiplier = 3;

    let calculation = {
        // has access to outside scope
        let value = 5 + 4;
        value * multiplier // DO NOT PUT A SEMICOLON HERE! 
    };

    println!("{calculation}")
}
