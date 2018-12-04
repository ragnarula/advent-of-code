use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let result = match File::open(filename) {
        Ok(file) => BufReader::new(file)
            .lines()
            .filter_map(|res| match res {
                Ok(line) => Some(line.chars().fold(HashMap::new(), |mut acc, c| {
                    *acc.entry(c).or_insert(0) += 1;
                    acc
                })),
                Err(_) => None,
            })
            .map(|acc| (acc.values().any(|&n| n == 2), acc.values().any(|&n| n == 3)))
            .map(|item| match item {
                (true, true) => (1, 1),
                (true, false) => (1, 0),
                (false, true) => (0, 1),
                (false, false) => (0, 0),
            })
            .fold((0, 0), |acc, item| (acc.0 + item.0, acc.1 + item.1)),
        Err(_) => (0, 0),
    };

    println!("Result: {}", result.0 * result.1)
}
