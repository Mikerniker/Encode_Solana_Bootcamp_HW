// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.
// NOTES: added &str to Vec<&str>


fn main() {
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}



// ORIGINAL

// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// fn main() {
//    let mut shopping_list: Vec<?> = Vec::new();
//    shopping_list.push("milk");
// }
