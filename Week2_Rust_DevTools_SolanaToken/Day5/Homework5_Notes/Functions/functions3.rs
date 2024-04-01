// functions3.rs
// Make me compile! Execute `rustlings hint functions3` for hints :)
// Notes: Add an argument to call_this e.g. from call_this()  to call_this(5)

fn main() {
    call_this(5);
}

fn call_this(num: u32) {
    for i in 0..num {
        println!("Loop now {}", i + 1);
    }
}