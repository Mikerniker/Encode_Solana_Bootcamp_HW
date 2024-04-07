// tests3.rs
// This test isn't testing our function -- make it do that in such a way that
// the test passes. Then write a second test that tests whether we get the result
// we expect to get when we call `is_even(5)`.
// NOTES: he assert! macro, provided by the standard library,
// is useful when you want to ensure that some condition in a test evaluates to true. W

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(4));   // check if bool = true
    }

    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(5));   // check if bool = false, add ! to negate
    }
}


// ORIGINAL

// tests3.rs
// This test isn't testing our function -- make it do that in such a way that
// the test passes. Then write a second test that tests whether we get the result
// we expect to get when we call `is_even(5)`.
// Execute `rustlings hint tests3` for hints :)


//pub fn is_even(num: i32) -> bool {
//    num % 2 == 0
//}

//#[cfg(test)]
//mod tests {
//    use super::*;

//    #[test]
//    fn is_true_when_even() {
//        assert!();
//    }

//    #[test]
//    fn is_false_when_odd() {
//        assert!();
//    }
//}