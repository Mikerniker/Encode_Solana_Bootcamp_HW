fn main() {
    println!("Welcome to FizzBuzz!");

    
    for num in 1..=301 {
        match (num % 3, num % 5) {
            (0, 0) => println!("Fizz Buzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", num),
        }
    }  // to do count number of time fizz buzz occcured 

   
}
