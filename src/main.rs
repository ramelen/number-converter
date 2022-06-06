/*
number conversion between numbers (just whole numbers) and a prime factorization based system
i call nampa (inspired by toki pona's wan tu mute)

it works like this:

take a number {42}

find a set of numbers the are all prime and sum to the number {2 * 3 * 7}

break down any that are larger into the prime factors of that number - 1 {2 * 3 * (2*3+1)}

rephrase to use implicit multiplication and remove the operators {2(3(2(3)1))}

all parentheses after the last number that isnt a one are closing, and visa versa, so they 
can be made straight {2|3|2|3|1||}

convert 1, 2, 3 to ·, :, ⋮ {:|⋮|:|⋮|·} or {:⋮:⋮·}

trailing lines are implied

bonus: if a number can be represented in a way other than its prime factorization 
like {2(2(2))1: 9} or {3(3)}, then the one can be moved out one more bracket.
so 17 is 2|2|2|2||1 ({:|:|:|:||·} or {}:::: ·}) -> 2(2(2(2))1) but 2(2(2))1 is 
not correct so -> 2(2(2()))1

to convert back:

take a number {:⋮:⋮·}

insert lines between every character {:|⋮|:|⋮|·}

replace lines with opening parens when before the last non 1 character {:(⋮(:(⋮)·}

close parens {:(⋮(:(⋮)·))}

replace with numbers {2(3(2(3)1))}

replace 2 and 3 with 2* and 3*, and 1 with +1 {2*(3*(2*(3)+1))}

evaluate left to right instead of the order of operations {2*3*7} -> {2*21} -> {42}

voila!
*/

use std::io; // input
use eval::{eval, Value}; // for converting back to a number

fn main() {
    // main loop: get input and parse to and from nampa
    loop {
        let mut number = String::new();

        // read input
        io::stdin()
            .read_line(&mut number)
            .expect("your numbers bad man");

        // convert from string to number
        let number = number.trim().parse().expect("this aint a number man");
        // convert to intermediate step
        let prime_factors = to_prime_factors(number, false);
        // convert to nampa
        let nampa = tawa_nampa(prime_factors.clone());
        println!("{nampa}");
        // convert intermediate step back to number as check (not full convert; 2 hour project)
        let number_reincarnated = to_number(to_prime_factors(number, true));
        // panic if the number doesnt convert back
        assert_eq!(number_reincarnated, number);
    }
}

fn to_prime_factors(number: usize, secret: bool) -> String {
    // makes an intermediate step between number and nampa, for checking
    let mut nampa = number; 
    let mut prime_factors: String = String::from("");
    let mut depth = 0; //number of factors, important for making closing brackets
    while nampa != 1 { // loop until all factors have been found
        // show all factorisation steps to make sure that the program isnt stuck
        for divisor in 2..=number { // guess and check for the lowest factor
            if nampa % divisor == 0 { // if divisible
                depth += 1; //increase the paren depth
                // two and three get their own symbol, but primes larger than that should be 
                // expressed as a non prime plus one
                if divisor > 3 {
                    let factors = to_prime_factors(divisor - 1, secret);
                    prime_factors.push_str(&(
                        //add parens to make compatible with order of operations in checking method
                        if secret { format!("({factors}1)(") }
                        else { format!("{factors}1(") } // add new factors to number
                    ));
                } else {
                    //add factor to number
                    prime_factors.push_str(&(format!("{divisor}(")));
                }
                nampa /= divisor; // remove divisor from number to avoid finding it muliple times
                break; // dont need to find any more divisors
            }
        }
    }
    prime_factors.push_str(&")".repeat(depth)); //make all the parens close
    prime_factors = prime_factors.replace("()", ""); // remove empty parens
    if prime_factors.is_empty() {prime_factors = String::from("1");}; // empty string must be one
    prime_factors //return string
}

fn tawa_nampa(nampa: String) -> String {
   let mut nampa: String = nampa
   .replace('(', "|") // opening parens are implicit
   .replace(')', "|") // closing parens are implicit
   .replace('1', "·") // replace with simpler symbol
   .replace('2', ":") // replace with simpler symbol
   .replace('3', "⋮");// replace with simpler symbol 

    while nampa.ends_with('|') {// remove trailing |
        let mut chars = nampa.chars();
        chars.next_back(); //remove the last char
        nampa = String::from(chars.as_str());
    };
    nampa // all done!
}

fn to_number(nampa: String) -> Value {
    let nampa = nampa // add operators
    .replace("3(", "3*(")
    .replace("2(", "2*(")
    .replace(")1)(", ")+1)*(")
    .replace(")1)", ")+1)");
    eval(&nampa).unwrap() //evaluate
}

#[cfg(test)]
mod tests { use super::*;
    #[test]
    fn verify() { // make sure that all numbers below 100000 convert correctly
        for number in 1..100_000 {
            assert_eq!(to_number(to_prime_factors(number, true)), number);
        }
    }
}