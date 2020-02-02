/// Represents four of possible states of fizzbuzz game.
///
/// Fizz is for numbers divisible by 3.
/// Buzz is for numbers divisible by 5.
/// FizzBuzz is for numbers divisible by 5 and 3.
/// None is for rest.
pub enum State {
    Fizz,
    Buzz,
    FizzBuzz,
    None(u32),
}

impl State {
    /// Constructor of fizzbuzz State enum.
    ///
    /// Takes single unsigned number and returns apropriate state.
    pub fn new(number: u32) -> State {
        match (number != 0, number % 3 == 0, number % 5 == 0) {
            (true, true, true) => State::FizzBuzz,
            (true, true, false) => State::Fizz,
            (true, false, true) => State::Buzz,
            _ => State::None(number),
        }
    }
}
