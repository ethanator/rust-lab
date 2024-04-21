// find_odd.rs
//
// Author: Yuxuan "Ethan" Chen
//
// Date: April 21, 2024
//
// Summary:
// This Rust program identifies the integer that appears an odd number of times
// within an array. Leveraging the XOR operation, it efficiently finds the sole
// number that does NOT cancel out when XORed with itself, which corresponds to
// the number appearing an odd number of times.
// It's designed to solve the problem posed on CodeWars:
// https://www.codewars.com/kata/54da5a58ea159efa38000836/train/rust
//
// Usage:
// Compile the program and run it.
//
// Example:
// $ ./find_odd
fn find_odd(arr: &[i32]) -> i32 {
    arr.iter().fold(0_i32, |a, v| a ^ v)
}

// fn find_odd_v2(arr: &[i32]) -> i32 {
//     let mut result = 0;
//     for &num in arr {
//         result ^= num;
//     }
//     result
// }

fn main() {
    println!("{}", find_odd(&[7])); // Outputs: 7
    println!("{}", find_odd(&[0])); // Outputs: 0
    println!("{}", find_odd(&[1, 1, 2])); // Outputs: 2
    println!("{}", find_odd(&[0, 1, 0, 1, 0])); // Outputs: 0
    println!("{}", find_odd(&[1, 2, 2, 3, 3, 3, 4, 3, 3, 3, 2, 2, 1])); // Outputs: 4

    // println!("{}", find_odd_v2(&[7])); // Outputs: 7
    // println!("{}", find_odd_v2(&[0])); // Outputs: 0
    // println!("{}", find_odd_v2(&[1, 1, 2])); // Outputs: 2
    // println!("{}", find_odd_v2(&[0, 1, 0, 1, 0])); // Outputs: 0
    // println!("{}", find_odd_v2(&[1, 2, 2, 3, 3, 3, 4, 3, 3, 3, 2, 2, 1])); // Outputs: 4
}
