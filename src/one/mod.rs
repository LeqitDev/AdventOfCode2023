/*
 * Welcome at Day 1
 *
 * Checkout the Challenge at https://adventofcode.com/2023/1.
 */

use std::{fs, ops::Index};

fn one(input: &str) {
    /* input
    .split('\n')
    .for_each(|b| b.chars().filter(|c| c as u8 >= 30 && c as u8 <= 39).); */
    let mut pairs: Vec<i32> = vec![];
    let digit_spell: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for block in input.split('\n') {
        let mut digits: Vec<(usize, i32)> = vec![];
        for (i, c) in block.chars().enumerate() {
            if c as u8 >= 48 && c as u8 <= 57 {
                digits.push((i, c.to_digit(10).unwrap() as i32));
            }
        }
        let digit_spell_clnd = digit_spell.clone();
        for (i, spelled) in digit_spell_clnd.iter().enumerate() {
            if let Some(x) = block.find(spelled) {
                if x < digits.first().unwrap_or(&(100000_usize, 0)).0 {
                    digits.splice(0..0, [(x, (i + 1) as i32)].iter().cloned());
                } else if x >= digits.last().unwrap_or(&(0_usize, 0)).0 {
                    digits.push((x, (i + 1) as i32));
                }
            }
        }
        // print!("{:?}, ", digits);
        pairs.push(
            digits.first().unwrap_or(&(0_usize, 0)).1 * 10
                + digits.last().unwrap_or(&(0_usize, 0)).1,
        );
    }
    let sum: i32 = pairs.iter().sum();
    println!("{}", sum);
}

pub fn one_test() {
    let input =
        fs::read_to_string("./src/one/example.txt").expect("Couldn't get the example input file!");
    one(input.as_str());
}

pub fn one_run() {
    let input =
        fs::read_to_string("./src/one/input.txt").expect("Couldn't get the real input file!");
    one(input.as_str());
}
