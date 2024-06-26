// vec2.rs
// A Vec of even numbers is given. Your task is to complete the loop
// so that each number in the Vec is multiplied by 2.


fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for i in v.iter_mut() {
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
        *i *= 2;     // used the * to dereference operator to get to the value in i to be able to use *= operator.
    }
    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}



// ORIGINAL


//fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
//    for i in v.iter_mut() {
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
//        *i *= 2;
//    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
//    v
//}

//#[cfg(test)]
//mod tests {
//    use super::*;

//    #[test]
//    fn test_vec_loop() {
//        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
//        let ans = vec_loop(v.clone());

  //      assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
  //  }
//}
