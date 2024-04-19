// count_by.rs
//
// Author: Yuxuan "Ethan" Chen
//
// Date: April 19, 2024
//
// Summary:
// This Rust program generates a vector of the first n multiples of a given
// number x.
// It's designed to solve the problem posed on CodeWars:
// https://www.codewars.com/kata/5513795bd3fafb56c200049e/train/rust
//
// Usage:
// Compile the program and run it, passing two positive integers as arguments:
// the first integer x is the multiplier, and the second integer n is the
// number of multiples.
//
// Example:
// $ ./count_by 1 10
// Outputs: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
// $ ./count_by 2 5
// Outputs: [2, 4, 6, 8, 10]
use std::env;
use std::process;

fn count_by(x: u32, n: u32) -> Vec<u32> {
    (1..=n).map(|i| x * i).collect()
}

// fn count_by_v2(x: u32, n: u32) -> Vec<u32> {
//     let mut multiples: Vec<u32> = Vec::new();
//     for i in 1..=n {
//         multiples.push(x * i);
//     }
//     multiples
// }

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: ./count_by <x> <n>");
        println!("Example:");
        println!("$ ./count_by 1 10");
        println!("Outputs: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]");
        println!("$ ./count_by 2 5");
        println!("Outputs: [2, 4, 6, 8, 10]");
        return;
    }

    let x = args[1].parse::<u32>().unwrap_or_else(|_| {
        println!(
            "Invalid input for x: {}. Please enter a positive integer.",
            args[1],
        );
        process::exit(1);
    });

    let n = args[2].parse::<u32>().unwrap_or_else(|_| {
        println!(
            "Invalid input for n: {}. Please enter a positive integer.",
            args[2],
        );
        process::exit(1);
    });

    println!("{:?}", count_by(x, n));
    // println!("{:?}", count_by_v2(x, n));
}
