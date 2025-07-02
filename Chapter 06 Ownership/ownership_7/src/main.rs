fn main() {
    /*
        String - A dynamic piece of text stored on the heap
        at runtime
        
        &String ("ref String") - A reference to a heap String
        
        str - A hardcoded, read-only piece of text encoded in
        the binary
        
        &str ("ref str") - A reference to the text in the memory
        that has loaded the binary file    
     */

    let ice_cream: &str = "Cookies and Cream"; // This is literally in binary in my executable. Very cool ngl
    println!("{}", ice_cream);
    let dessert: &str = ice_cream; // there is no movement of ownership. Because it's a reference, it's just a pointer to the same memory location
    println!("{}", ice_cream); // Still works
    println!("{} {}",dessert, ice_cream); // Both work
}
