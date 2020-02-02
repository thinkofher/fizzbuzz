fizzbuzz
-------

According to [Wikipedia](https://en.wikipedia.org/wiki/Fizz_buzz) `Fizz buzz` is:

> Fizz buzz is a group word game for children to teach them about division. Players take turns to count incrementally, replacing any number divisible by three with the word "fizz", and any number divisible by five with the word "buzz".

`fizzbuzz`, in turn, is a command line tool to print all Fizz Buzz game entries from 1 to given range.

Requirements
------------

You just need `cargo`, `rust` package manager.


Installation
------------

Clone this repo, enter the directory and run:

    $ cargo install --path .

If you have `$HOME/.cargo/bin` string in your `$PATH` envrioment variable, you can start using `fizzbuzz`!

Sample usage
------------

Try using `fizzbuzz` with some natural numbers:

    $ fizzbuzz 5
    1
    2
    Fizz!
    4
    Buzz!

If you are user of some unix-like operating system, you can try to use it with other unixy tools.

    $ fizzbuzz 15 | nl | grep -i "buzz\|fizz"
         3    Fizz!
         5    Buzz!
         6    Fizz!
         9    Fizz!
        10    Buzz!
        12    Fizz!
        15    FizzBuzz!

Cool!

Why
---

Because of fun and because of [this](https://youtu.be/QPZ0pIK_wsc).
