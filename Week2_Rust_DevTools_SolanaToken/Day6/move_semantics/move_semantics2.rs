// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// NOTES fn fill_vec(vec: Vec<i32>) -> Vec<i32> [this parameter takes ownership of the value ]
// CHANGE Clone the value, changed:  let mut vec1 = fill_vec(vec0); to let mut vec1 = fill_vec(vec0.clone());

fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0.clone());

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
