use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = File::open(filename)?;

    let mut sum = 0;

    for line in BufReader::new(file).lines() {
        let strline = line.unwrap();
        let op = strline.char_indices().nth(0).unwrap().1;
        let val = &strline[1..].parse::<i32>().unwrap();
        match op {
            '+' => sum = sum + val,
            '-' => sum = sum - val,
            _ => println!("Invalid op"),
        }
    }

    println!("Result: {}", sum);
    Ok(())
}
