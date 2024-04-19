// are_you_playing_banjo.rs
//
// Author: Yuxuan "Ethan" Chen
//
// Date: April 19, 2024
//
// Summary:
// This Rust program determines if a person plays the banjo based on
// their name.
// If the name starts with the letter 'R' or 'r', the person is said
// to play the banjo.
// It's designed to solve the problem posed on CodeWars:
// https://www.codewars.com/kata/53af2b8861023f1d88000832/train/rust
//
// Usage:
// Compile the program and run it, passing a string as an argument.
//
// Example:
// $ ./are_you_playing_banjo "Ethan"
// Outputs: Ethan does not play banjo
// If no arguments are provided, the program will prompt the user to
// provide an input string.
use std::env;

fn are_you_playing_banjo(name: &str) -> String {
    match &name[0..1] {
        "R" | "r" => format!("{} plays banjo", name),
        _ => format!("{} does not play banjo", name),
    }
}

// fn are_you_playing_banjo_v2(name: &str) -> String {
//     if name.to_lowercase().starts_with('r') {
//         format!("{} plays banjo", name)
//     } else {
//         format!("{} does not play banjo", name)
//     }
// }

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        println!("{}", are_you_playing_banjo(&args[1]));
        // println!("{}", are_you_playing_banjo_v2(&args[1]));
    } else {
        println!("Please provide an input string.");
    }
}
