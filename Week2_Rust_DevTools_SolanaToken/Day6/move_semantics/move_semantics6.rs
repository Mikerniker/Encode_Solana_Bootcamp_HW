// move_semantics6.rs
// Make me compile! `rustlings hint move_semantics6` for hints
// You can't change anything except adding or removing references



fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);   //changed added &

    string_uppercase(data);  // change: removed &
}

// Should not take ownership
fn get_char(data: &String) -> char {  //change: added &
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {  //change: removed &
    data = data.to_uppercase();   //change: removed &

    println!("{}", data);
}





// ORIGINAL
//fn main() {
  //  let data = "Rust is great!".to_string();

//    get_char(data);

//    string_uppercase(&data);
// }

// Should not take ownership
// fn get_char(data: String) -> char {
  //  data.chars().last().unwrap()
// }

// Should take ownership
// fn string_uppercase(mut data: &String) {
   // data = &data.to_uppercase();

//    println!("{}", data);
// }
