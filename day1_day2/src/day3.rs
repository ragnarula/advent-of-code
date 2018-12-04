use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Point {
    x: u32,
    y: u32,
}

struct Rectangle {
    top_left: Point,
    bot_right: Point,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // read each line, mark spot in 2 dim array as occupied
    // iterate over 2 dim array and count which ones have more than one occupied
}
