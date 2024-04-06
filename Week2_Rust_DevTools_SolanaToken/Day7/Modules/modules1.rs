// modules1.rs
//


mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {  // don't add pub to keep it private
        String::from("Ginger")
    }

    pub fn make_sausage() {   //add pub
        get_secret_recipe();
        println!("sausage!");
    }
}

// ORIGINAL

// modules1.rs

//mod sausage_factory {
    // Don't let anybody outside of this module see this!
//    fn get_secret_recipe() -> String {
//        String::from("Ginger")
//    }

//    fn make_sausage() {
//        get_secret_recipe();
//        println!("sausage!");
//    }
//}

//fn main() {
//    sausage_factory::make_sausage();
//}
