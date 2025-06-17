fn main() {
    let coffee_price = 5.99;
    /* 
    This
    is
    a
    scope.
    */
    {
        /* Inner scope. I have access to variables in the outer scope. */
        let cookie_price = 1.99; // Has no use since in inner scope.
        let coffee_price = 1.99;
        println!("The price of coffee is: {coffee_price} USD."); // Price is 1.99
    }
    // println!("The price of a cookie is : {cookie_price} USD."); -Here cookie price is nonexistant. Because it was in a nested inner scope block.
    println!("The price of coffee is : {coffee_price} USD."); // See how the output is different? Awesome nesting stuff! :3
}
