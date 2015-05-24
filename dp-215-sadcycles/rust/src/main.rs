use std::io;
use std::string::String;
use std::iter::Iterator;

fn main() {
    println!("Hello, sailor!  Give me some numbers.");

    // read input values
    println!("What's the base?");
    let mut base = String::new();
    io::stdin().read_line(&mut base).unwrap();
    let base: u32 = base.trim().parse().unwrap();

    println!("Okay, so then what's our starting number?");
    let mut number = String::new();
    io::stdin().read_line(&mut number).unwrap();
    number = number.trim().to_string();

    let mut cycle = Vec::new();                         // A vector to hold all of the cycled numbers
    cycle.push(number);                                 // Push in our starting value
    let mut index = 0;

    loop {
        let mut sum: u32 = 0;
        let n = cycle.last().unwrap().clone();          // The last number calculated in the cylce1
        let numbers = n.chars();                        // Converting the string to a vector of characters
        for n in numbers {
            sum += n.to_digit(10).unwrap().pow(base);   // Raise each character (as a digit) to our base
        }
        if cycle.contains(&sum.to_string()) {           // If the sum is already in our cycle, we've looped
            index = cycle.iter().position(|x| *x == *sum.to_string()).unwrap();
            break;
        }

        cycle.push(sum.to_string());                    // Otherwise, add the unique number to the cycle and keep counting.
    }

    let (null, pattern) = cycle.split_at(index);        // Index - 1 because split_at puts the index in the first split

    for x in 0..pattern.len()  {
        if x == pattern.len() - 1 {
            println!("{0} ", pattern[x]);
        } else {
            print!("{0}, ", pattern[x]);
        }
    }
}
