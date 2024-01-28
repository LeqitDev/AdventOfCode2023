/*
 * Welcome at Day 6
 *
 * Checkout the Challenge at https://adventofcode.com/2023/6.
 */

use std::fs;

fn six(input: &str) {}

pub fn six_test() {
    let input =
        fs::read_to_string("./src/six/example.txt").expect("Couldn't get the example input file!");
    six(input.as_str());
}

pub fn six_run() {
    let input =
        fs::read_to_string("./src/six/input.txt").expect("Couldn't get the real input file!");
    six(input.as_str());
}
