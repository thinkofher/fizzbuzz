/// Represents four of possible states of fizzbuzz game.
///
/// Fizz is for numbers divisible by 3.
/// Buzz is for numbers divisible by 5.
/// FizzBuzz is for numbers divisible by 5 and 3.
/// None is for rest.
pub enum FizzBuzz {
    Fizz,
    Buzz,
    FizzBuzz,
    None(u32),
}

impl FizzBuzz {
    /// Constructor of FizzBuzz enum.
    ///
    /// Takes single unsigned number and returns apropriate state.
    pub fn new(number: u32) -> FizzBuzz {
        match (number != 0, number % 3 == 0, number % 5 == 0) {
            (true, true, true) => FizzBuzz::FizzBuzz,
            (true, true, false) => FizzBuzz::Fizz,
            (true, false, true) => FizzBuzz::Buzz,
            _ => FizzBuzz::None(number),
        }
    }
}
