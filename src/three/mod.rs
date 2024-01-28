/*
 * Welcome at Day 3
 *
 * Checkout the Challenge at https://adventofcode.com/2023/3.
 */

use std::{collections::HashMap, fs};

fn three(input: &str) {
    let line_split: Vec<&str> = input.split('\n').collect();
    let line_length = line_split.first().unwrap().len();
    let numbers: Vec<char> = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
    let mut numbers_dots = numbers.clone();
    numbers_dots.push('.');
    numbers_dots.push('\n');
    println!("Length: {}", line_length);

    let mut found_numbers: Vec<((i32, i32), char)> = vec![];
    let mut found_symbols: Vec<((i32, i32), char)> = vec![];

    for (i, c) in input.replace('\n', "").chars().enumerate() {
        // if !numbers_dots.contains(&c) {
        if c == '*' {
            let x = i % line_length;
            let y = i / line_length;
            found_symbols.push(((x as i32, y as i32), c));
        } else if numbers.contains(&c) {
            let x = i % line_length;
            let y = i / line_length;
            found_numbers.push(((x as i32, y as i32), c));
        }
    }

    let coords: Vec<(i32, i32)> = vec![
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
    ];

    let mut x = 0;
    let mut y = 0;
    let mut s = "".to_string();
    let mut add = false;
    // let mut sum = 0;

    let mut m: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
    let mut index: (i32, i32) = (0, 0);
    for ((cur_x, cur_y), c) in found_numbers {
        // println!("{}: {}:{}, {}:{}; - {}", c, x, cur_x, y, cur_y, s);
        if cur_x - x <= 1 && y == cur_y {
            s.push(c);
            x = cur_x;
        } else {
            if add {
                let num = s.parse::<i32>().unwrap_or(0);
                // println!("{}", num);
                // sum += num;
                if let Some(v) = m.get_mut(&index) {
                    v.push(num);
                } else {
                    m.insert(index, vec![num]);
                }
                add = false;
            }

            s.clear();
            s.push(c);
            x = cur_x;
            y = cur_y;
        }

        for (coord_x, coord_y) in &coords {
            if found_symbols.iter().any(|((s_x, s_y), _)| {
                if *s_x == cur_x + *coord_x && *s_y == cur_y + *coord_y {
                    index = (*s_x, *s_y);
                    return true;
                }
                false
            }) {
                add = true;
            }
        }
    }
    if add {
        let num = s.parse::<i32>().unwrap_or(0);
        // println!("{}", num);
        // sum += num;
        if let Some(v) = m.get_mut(&index) {
            v.push(num);
        } else {
            m.insert(index, vec![num]);
        }
    }

    // println!("{:?}", found_symbols);

    println!(
        "{}",
        m.iter()
            .filter(|(_, v)| v.len() == 2)
            .map(|(_, v)| v[0] * v[1])
            .sum::<i32>()
    );
}

/* fn get_symbol_at_coords(x: usize, y: usize, input: &str, line_length: usize) -> char {
    input.chars().collect::<Vec<char>>()[x + y * line_length]
}

fn get_next_number(index: usize, input: &str, line_length: usize) -> (i32, usize, usize, usize) {
    for c in input.splitn(index, "").collect::<Vec<&str>>()[1].chars() {

    }
} */

pub fn three_test() {
    let input = fs::read_to_string("./src/three/example.txt")
        .expect("Couldn't get the example input file!");
    three(input.as_str());
}

pub fn three_run() {
    let input =
        fs::read_to_string("./src/three/input.txt").expect("Couldn't get the real input file!");
    three(input.as_str());
}
