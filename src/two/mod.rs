/*
 * Welcome at Day 2
 *
 * Checkout the Challenge at https://adventofcode.com/2023/2.
 */

use std::{collections::HashMap, fs};

fn two(input: &str) {
    let mut id_sum = 0;
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        let split = line.split(": ").collect::<Vec<&str>>();
        let mut id = 0;
        split[0].split(' ').collect::<Vec<&str>>()[1]
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .rev()
            .enumerate()
            .for_each(|(i, d)| id += d as i32 * 10_i32.pow(i as u32));
        let mut cube_count: HashMap<&str, i32> = HashMap::new();
        cube_count.insert("red", -1);
        cube_count.insert("blue", -1);
        cube_count.insert("green", -1);
        split[1].split("; ").for_each(|s| {
            s.split(", ").for_each(|cube| {
                let args: Vec<&str> = cube.split(" ").collect();
                match cube_count.get_mut(args[1]) {
                    Some(value) => {
                        let mut count = 0;
                        args[0]
                            .chars()
                            .map(|c| c.to_digit(10).unwrap())
                            .rev()
                            .enumerate()
                            .for_each(|(i, d)| count += d as i32 * 10_i32.pow(i as u32));

                        if count > *value {
                            *value = count;
                        }
                    }
                    None => (),
                }
            })
        });
        // if *cube_count.get("red").unwrap() <= 12 && *cube_count.get("green").unwrap() <= 13 && *cube_count.get("blue").unwrap() <= 14 {id_sum += id} else {
        //     println!("{:?}", cube_count);
        // }
        id_sum += (*cube_count.get("red").unwrap()
            * *cube_count.get("green").unwrap()
            * *cube_count.get("blue").unwrap());
    }
    println!("\n{}", id_sum);
}

pub fn two_test() {
    let input =
        fs::read_to_string("./src/two/example.txt").expect("Couldn't get the example input file!");
    two(input.as_str());
}

pub fn two_run() {
    let input =
        fs::read_to_string("./src/two/input.txt").expect("Couldn't get the real input file!");
    two(input.as_str());
}
