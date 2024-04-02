// variables5.rs
// NOTES: Shadowing, need to change the second:  number = 3 to let number = 3 bec it is an integer not string


fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
}