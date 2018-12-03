use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let lines: Vec<String> = match File::open(filename) {
        Ok(file) => BufReader::new(file)
            .lines()
            .filter_map(|res| match res {
                Ok(a) => Some(a),
                Err(_) => None,
            })
            .collect(),
        Err(_) => Vec::new(),
    };

    let perm: String = lines
        .iter()
        .flat_map(|i_line| lines.iter().map({ move |j_line| (i_line, j_line) }))
        .map(|line_pair| line_pair.0.chars().zip(line_pair.1.chars()).collect())
        .map(
            |char_pairs: Vec<(char, char)>| -> (i32, Vec<(char, char)>) {
                (
                    char_pairs.iter().fold(0, |n, pair| match pair {
                        (a, b) if a != b => n + 1,
                        _ => n,
                    }),
                    char_pairs,
                )
            },
        )
        .filter(|diff_and_char_pairs| diff_and_char_pairs.0 == 1)
        .take(1)
        .map(|diff_and_char_pairs| diff_and_char_pairs.1)
        .map(|char_pairs| -> String {
            char_pairs
                .iter()
                .filter_map(|pair| match pair {
                    (a, b) if a == b => Some(a),
                    _ => None,
                })
                .collect()
        })
        .collect();

    println!("{}", perm);

    // for i_line in &lines {
    //     for j_line in &lines {
    //         let paired: Vec<(char, char)> = i_line.chars().zip(j_line.chars()).collect();
    //         let num_diff = paired.iter().fold(0, |n, pair| match pair {
    //             (a, b) if a != b => n + 1,
    //             (_, _) => n,
    //         });

    //         if num_diff == 1 {
    //             let common: String = paired
    //                 .iter()
    //                 .filter(|(a, b)| a == b)
    //                 .map(|pair| pair.0)
    //                 .collect();

    //             println!("{}", common);
    //         }
    //     }
    // }
}
