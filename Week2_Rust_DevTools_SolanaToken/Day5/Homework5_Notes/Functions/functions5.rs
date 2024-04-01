// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)

// Notes: Modified num * num; to num * num  (without semicolon) bec
// If you add a semicolon to the end of an expression,
// you turn it into a statement, and it will then not return a value.

fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}