// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// From references and borrowing chapter:  use curly brackets
// to create a new scope, allowing for multiple mutable references, just not simultaneous ones:


fn main() {
    let mut x = 100;

    {
    let y = &mut x;
    *y += 100;
    }

    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);

}



// ORIGINAL:
// fn main() {
//     let mut x = 100;
//     let y = &mut x;
//     let z = &mut x;
//     *y += 100;
//     *z += 1000;
//     assert_eq!(x, 1200);
// }