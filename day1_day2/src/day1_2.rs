use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let parsed_lines: Vec<(char, i32)> = match File::open(filename) {
        Ok(file) => BufReader::new(file)
            .lines()
            .map(|res| -> (Option<char>, Option<i32>) {
                match res {
                    Ok(line) => (line.chars().nth(0), line[1..].parse::<i32>().ok()),
                    Err(_) => (None, None),
                }
            })
            .filter_map(|item| match item {
                (Some(a), Some(b)) => Some((a, b)),
                _ => None,
            })
            .collect(),
        Err(_) => Vec::new(),
    };

    let mut sums = HashSet::<i32>::new();
    let mut sum = 0;
    let mut done = false;

    while !done {
        for line in &parsed_lines {
            match line.0 {
                '+' => sum = sum + line.1,
                '-' => sum = sum - line.1,
                _ => println!("Invalid op"),
            }

            if sums.contains(&sum) {
                println!("Result: {}", sum);
                done = true;
                break;
            }

            sums.insert(sum);
        }
    }
}
