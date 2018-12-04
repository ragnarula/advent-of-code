fn parse_line(line: &str) -> Option<(u32, (u32, u32), (u32, u32), (u32, u32))> {
    let split_line: Vec<&str> = line.split(' ').collect();
    let id = split_line
        .get(0)
        .and_then(|id_col| id_col.get(1..))
        .and_then(|id_str| id_str.parse::<u32>().ok());

    let top_left = split_line
        .get(2)
        .map(|tl_str| {
            tl_str
                .split(',')
                .map(|x| x.trim_end_matches(':').to_string())
                .collect()
        })
        .map(|tl_split: Vec<String>| {
            (
                tl_split.get(0).and_then(|x| x.parse::<u32>().ok()),
                tl_split.get(1).and_then(|x| x.parse::<u32>().ok()),
            )
        })
        .and_then(|pair| match pair {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        });

    let size = split_line
        .get(3)
        .map(|size_str| size_str.split('x').map(|x| x.to_string()).collect())
        .map(|size_split: Vec<String>| {
            (
                size_split.get(0).and_then(|x| x.parse::<u32>().ok()),
                size_split.get(1).and_then(|x| x.parse::<u32>().ok()),
            )
        })
        .and_then(|pair| match pair {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        });

    let bot_right = match (top_left, size) {
        (Some(tl), Some(sz)) => Some((tl.0 + sz.0 - 1, tl.1 + sz.1 - 1)),
        _ => None,
    };

    let result = match (id, size, top_left, bot_right) {
        (Some(a), Some(b), Some(c), Some(d)) => Some((a, b, c, d)),
        _ => None,
    };

    result
}

fn get_index(row_col: (u32, u32)) -> usize {
    ((row_col.0 * 1000) + row_col.1) as usize
}

fn box_intersects(a: &((u32, u32), (u32, u32)), b: &((u32, u32), (u32, u32))) -> bool {
    !((b.0).0 > (a.1).0 || (b.1).0 < (a.0).0 || (b.0).1 > (a.1).1 || (b.1).1 < (a.0).1)
}

fn part_1_solve(input: &str) -> usize {
    let mut table: Vec<u32> = vec![0; 1000 * 1000];

    let lines: Vec<(u32, (u32, u32), (u32, u32), (u32, u32))> =
        input.lines().filter_map(|l| parse_line(l)).collect();

    for line in lines {
        let top_left = line.2;
        let bot_right = line.3;

        for i in top_left.0..=bot_right.0 {
            for j in top_left.1..=bot_right.1 {
                table[get_index((i, j))] += 1;
            }
        }
    }

    let mut count = 0;
    for i in &table {
        if *i > 1 {
            count += 1;
        }
    }
    count
}

fn part_2_solve(input: &str) -> usize {
    let lines: Vec<(u32, (u32, u32), (u32, u32), (u32, u32))> =
        input.lines().filter_map(|l| parse_line(l)).collect();

    for i_line in &lines {
        let mut did_intersect = false;

        for j_line in &lines {
            let box_i = (i_line.2, i_line.3);
            let box_j = (j_line.2, j_line.3);

            if did_intersect == false && i_line.0 != j_line.0 {
                did_intersect = box_intersects(&box_i, &box_j);
            }
        }

        if did_intersect == false {
            return i_line.0 as usize;
        }
    }

    unreachable!()
}

fn main() {
    println!(
        "Part 1 - {}",
        part_1_solve(include_str!("../data/input.txt"))
    );

    println!(
        "Part 2 - {}",
        part_2_solve(include_str!("../data/input.txt"))
    );
}

#[test]
fn test_part_1() {
    assert_eq!(part_1_solve(include_str!("../data/test1.txt")), 4);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2_solve(include_str!("../data/test1.txt")), 3);
}

#[test]
fn parse_line_test() {
    assert_eq!(
        parse_line("#1 @ 1,3: 4x4"),
        Some((1, (4, 4), (1, 3), (4, 6)))
    );

    assert_eq!(
        parse_line("#123 @ 3,2: 5x4"),
        Some((123, (5, 4), (3, 2), (7, 5)))
    );
}

#[test]
fn box_intersect_test() {
    let box_a = ((1, 1), (3, 3));
    let box_b = ((4, 4), (7, 7));
    let box_c = ((2, 2), (5, 5));

    assert!(box_intersects(&box_a, &box_c));
    assert!(!box_intersects(&box_a, &box_b));
    assert!(box_intersects(&box_b, &box_c));
}
