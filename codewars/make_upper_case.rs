// make_upper_case.rs
//
// Author: Yuxuan "Ethan" Chen
//
// Date: April 19, 2024
//
// Summary:
// This Rust program reads a string from the command line arguments and
// converts it to uppercase.
// It's designed to solve the problem posed on CodeWars:
// https://www.codewars.com/kata/57a0556c7cb1f31ab3000ad7/train/rust
//
// Usage:
// Compile the program and run it, passing a string as an argument.
//
// Example:
// $ ./make_upper_case "hello world"
// Outputs: HELLO WORLD
// If no arguments are provided, the program will prompt the user to
// provide an input string.
use std::env;

fn make_upper_case(s: &str) -> String {
    s.to_uppercase()
}

// static MAKE_UPPER_CASE_V2: fn(&str) -> String = str::to_uppercase;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        println!("{}", make_upper_case(&args[1]));
        // println!("{}", MAKE_UPPER_CASE_V2(&args[1]));
    } else {
        println!("Please provide an input string.");
    }
}
