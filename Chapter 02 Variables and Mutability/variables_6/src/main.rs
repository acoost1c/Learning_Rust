#![allow(unused_variables)] // Global directive for the WHOLE FILE!!!

type Meters = i32; // Provides context to the reader of code.

#[allow(unused_variables)] // The whole funtion gets an excuse.
fn main() {
    #[allow(unused_variables)] // Compiler directive, making excuses for the NEXT line.
    let mile_race_length: Meters = 1600; // Data type already specified.
    #[allow(unused_variables)]
    let two_mile_race_length: Meters =3200;
}
