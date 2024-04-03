// move_semantics3.rs
// Make me compile without adding new lines-- just changing existing lines!
// (no lines with multiple semicolons necessary!)
// fn fill_vec(vec: Vec<i32>) -> Vec<i32>    is not mutable
// The following cannot borrow as mutable:  vec.push(22); vec.push(44);, and vec.push(66);


fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {   // changed here
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
