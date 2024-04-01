fn main() {
    println!("Welcome to FizzBuzz!");

    let mut fizz_buzz_count = 0;
    for num in 1..=301 {
        match (num % 3, num % 5) {
            (0, 0) => {println!("Fizz Buzz"); fizz_buzz_count += 1;},
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", num),
        }
    }
    println!("Total 'Fizz Buzz': {fizz_buzz_count}")
   
}
