// learning traits
fn main() {
        /* Display trait */
    let seasons: [&str; 4] = ["Spring","Summer","Fall","Winter"];
    println!("{}", 5);
    println!("{}", 3.14);
    println!("{}", true);
    // println!("{}", seasons); !!! can't use the display trait with an array.
    
        /* Debug trait */
    println!("{seasons:?}"); // with the :? debug trait we can see the whole array
    println!("{seasons:#?}"); // this # is a pretty print
    
        /* Debug macro */
    dbg!(2 + 2); // shows everything you need, and how it does it
    dbg!(seasons); // faster way to see whats going on without the prontln etc
}
