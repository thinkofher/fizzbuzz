use std::env;

mod fizzbuzz;
use fizzbuzz::FizzBuzz;

fn main() {
    // Skip first argument, because it is just name of the executable
    let args: Vec<String> = env::args().skip(1).collect();

    let start = 1; // start of range

    // Parse end of range from command line arguments
    let end: u32 = args
        .first()
        .unwrap_or_else(|| {
            eprintln!("You have to provide first argument as the value of end of range!");
            std::process::exit(1);
        })
        .parse()
        .unwrap_or_else(|_| {
            eprintln!("End of range must be a natural integer.");
            std::process::exit(1);
        });

    // Show all numbers from one to end of the range
    for number in start..=end {
        match FizzBuzz::new(number) {
            FizzBuzz::FizzBuzz => println!("FizzBuzz!"),
            FizzBuzz::Fizz => println!("Fizz!"),
            FizzBuzz::Buzz => println!("Buzz!"),
            FizzBuzz::None(number) => println!("{}", number),
        }
    }
}
