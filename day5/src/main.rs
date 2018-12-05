fn main() {
    println!(
        "Part 1 Result - {}",
        solve_1(include_str!("../data/input.txt"))
    );
    println!(
        "Part 2 Result - {}",
        solve_2(include_str!("../data/input.txt"))
    );
}

fn does_react(a: char, b: char) -> bool {
    (a.is_ascii_lowercase() && b.is_ascii_uppercase() && a == b.to_ascii_lowercase())
        || (a.is_ascii_uppercase() && b.is_ascii_lowercase() && a.to_ascii_lowercase() == b)
}

fn reduce(input: &str) -> String {
    let mut result: (Option<char>, String) =
        input
            .chars()
            .fold((None, String::new()), |acc, c| match acc {
                (None, s) => (Some(c), s),
                (Some(p), ref s) if does_react(p, c) => (None, s.clone()),
                (Some(p), s) => {
                    let mut new_string = s.clone();
                    new_string.push(p);
                    (Some(c), new_string)
                }
            });

    match result.0 {
        Some(c) => result.1.push(c),
        None => {}
    }
    result.1
}

fn solve_1(input: &str) -> usize {
    let mut length = 0;
    let mut result = input.to_owned();
    loop {
        result = reduce(&result);
        let reduced_length = result.len();
        if reduced_length == length {
            break;
        }
        length = reduced_length;
    }

    length
}

fn solve_2(input: &str) -> usize {
    let types = "abcdefghijklmnopqrstuvwxyz";
    let min: Option<usize> = types
        .chars()
        .map(|c| {
            let first = input.to_string().replace(c, "");
            first.replace(c.to_ascii_uppercase(), "")
        })
        .map(|s| solve_1(&s))
        .min();
    min.unwrap()
}

#[test]
fn test_does_react() {
    assert!(does_react('a', 'A'));
    assert!(does_react('A', 'a'));
    assert!(!does_react('a', 'a'));
    assert!(!does_react('A', 'A'));
    assert!(!does_react('a', 'B'));
    assert!(!does_react('A', 'b'));
}

#[test]
fn test_reduce() {
    assert_eq!(reduce("dabAcCaCBAcCcaDA"), "dabAaCBAcaDA");
    assert_eq!(reduce("dabAaCBAcaDA"), "dabCBAcaDA");
    assert_eq!(reduce("dabCBAcaDA"), "dabCBAcaDA");
}

#[test]
fn test_solve_1() {
    assert_eq!(solve_1("dabAcCaCBAcCcaDA"), 10);
}

#[test]
fn test_solve_2() {
    assert_eq!(solve_2("dabAcCaCBAcCcaDA"), 4);
}
