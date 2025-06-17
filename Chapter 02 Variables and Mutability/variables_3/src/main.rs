fn main() {
    // Variable shadowing:
    let grams_of_protein = "100.345";
    // Here the variable is a String.
    let grams_of_protein = 100.345;
    // Here the variable is a float point number.
    let mut grams_of_protein = 100;
    // Here the variable is an integer number. And becomes mutable!
    // Them mfs "shadow" each other, kinda cool! :D
    grams_of_protein = 105; // I can now change it, but only in integers.
}
