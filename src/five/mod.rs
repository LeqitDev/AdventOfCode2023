/*
 * Welcome at Day 5
 *
 * Checkout the Challenge at https://adventofcode.com/2023/5.
 */

use std::fs;

fn five(input: &str) {}

pub fn five_test() {
    let input =
        fs::read_to_string("./src/five/example.txt").expect("Couldn't get the example input file!");
    five(input.as_str());
}

pub fn five_run() {
    let input =
        fs::read_to_string("./src/five/input.txt").expect("Couldn't get the real input file!");
    five(input.as_str());
}
