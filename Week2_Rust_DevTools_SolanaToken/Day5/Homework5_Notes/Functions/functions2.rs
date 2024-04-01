// functions2.rs
// Make me compile! Execute `rustlings hint functions2` for hints :)
// Notes: Add type e.g. from call_this(num) to call_this(num: i32)

fn main() {
    call_this(3);
}

fn call_this(num: i32) {
    for i in 0..num {
        println!("Loop! number {}", i + 1);
    }
}
