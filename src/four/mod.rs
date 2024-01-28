/*
 * Welcome at Day 4
 *
 * Checkout the Challenge at https://adventofcode.com/2023/4.
 */

use std::{collections::HashMap, fs};

fn four(input: &str) {
    // let mut sum = 0;
    let mut m: HashMap<i32, i32> = HashMap::new();
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }

        let game = line.split(": ").collect::<Vec<&str>>();
        let id = game[0].split(' ').collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap_or(-1);
        let m_c = m.clone();
        let game_count = m_c.get(&id);

        let split = game[1].split(" | ").collect::<Vec<&str>>();
        let winning_numbers: Vec<i32> = split[0]
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let cards: Vec<i32> = split[1]
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let mut i = 0;

        for card in cards {
            if winning_numbers.contains(&card) {
                i += 1;
            }
        }

        /* print!("{:?}, ", i);
        if i != 0 {
            sum += 2_i32.pow(i - 1);
        } */
        for c in (1..i + 1).collect::<std::vec::Vec<i32>>() {
            if let Some(g) = m.get_mut(&(id + c)) {
                *g += if let Some(count) = game_count {
                    count
                } else {
                    &0
                }
            } else {
                m.insert(id + c, 2);
            }
        }

        if m_c.get(&id).is_none() {
            m.insert(id, 1);
        }
    }
    println!("{:?}", m.values().sum::<i32>());
    // println!("{}", sum);
}

pub fn four_test() {
    let input =
        fs::read_to_string("./src/four/example.txt").expect("Couldn't get the example input file!");
    four(input.as_str());
}

pub fn four_run() {
    let input =
        fs::read_to_string("./src/four/input.txt").expect("Couldn't get the real input file!");
    four(input.as_str());
}
