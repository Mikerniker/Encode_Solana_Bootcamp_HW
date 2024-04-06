// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
// Changed by adding <T> or T to support any type

struct Wrapper<T> {    // Added <T>
    value: T,          // Here
}

impl<T> Wrapper<T> {   // Here
    pub fn new(value: T) -> Self {   // and Here
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}



//ORIGINAL
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.


//struct Wrapper {
//    value: u32,
//}

//impl Wrapper {
//    pub fn new(value: u32) -> Self {
//        Wrapper { value }
//    }
//}

//#[cfg(test)]
//mod tests {
//    use super::*;

//    #[test]
//    fn store_u32_in_wrapper() {
//        assert_eq!(Wrapper::new(42).value, 42);
//    }

//    #[test]
//    fn store_str_in_wrapper() {
//        assert_eq!(Wrapper::new("Foo").value, "Foo");
//    }
//}
