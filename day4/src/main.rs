extern crate chrono;
use chrono::DateTime;
use chrono::*;
use std::cmp::Ordering;
use std::collections::HashMap;

struct GuardEntry {
    id: u32,
    schedule: [u32; 60],
}

impl GuardEntry {
    fn new(id: u32) -> GuardEntry {
        GuardEntry {
            id: id,
            schedule: [0; 60],
        }
    }

    fn get_sleep_mins(&self) -> u32 {
        let mut sum = 0;
        for i in 0..60 {
            sum += self.schedule[i];
        }
        sum
    }

    fn get_max_minute(&self) -> u32 {
        let mut max_value = 0;
        let mut max_i = 0;
        for i in 0..60 {
            if self.schedule[i] > max_value {
                max_i = i;
                max_value = self.schedule[i];
            }
        }
        max_i as u32
    }

    fn get_max_minute_value(&self) -> u32 {
        self.schedule[self.get_max_minute() as usize]
    }
}

struct ResultAccumulator {
    current_guard: u32,
    sleep_mins: u32,
    guard_table: HashMap<u32, GuardEntry>,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
enum LogEntry {
    ShiftBegin { id: u32 },
    Sleep,
    WakeUp,
}

struct ParsedLine {
    timestamp: DateTime<Utc>,
    entry: LogEntry,
}

impl Ord for ParsedLine {
    fn cmp(&self, other: &ParsedLine) -> Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl PartialOrd for ParsedLine {
    fn partial_cmp(&self, other: &ParsedLine) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ParsedLine {
    fn eq(&self, other: &ParsedLine) -> bool {
        self.timestamp == other.timestamp
    }
}

impl Eq for ParsedLine {}

fn parse_line(input: &str) -> Option<ParsedLine> {
    let split_line = input.split_at(18);

    let split_entry: Vec<&str> = split_line.1.trim().split(' ').collect();

    let entry = match split_entry.get(0) {
        Some(&"falls") => Some(LogEntry::Sleep),
        Some(&"wakes") => Some(LogEntry::WakeUp),
        Some(&"Guard") => split_entry
            .get(1)
            .map(|&s| &s[1..])
            .and_then(|s| s.parse::<u32>().ok())
            .map(|u| LogEntry::ShiftBegin { id: u }),
        _ => None,
    };

    let timestamp = Utc.datetime_from_str(split_line.0, "[%Y-%m-%d %H:%M]").ok();

    match (entry, timestamp) {
        (Some(e), Some(t)) => Some(ParsedLine {
            timestamp: t,
            entry: e,
        }),
        _ => None,
    }
}

fn solve_1(input: &str) -> usize {
    let mut parsed_lines: Vec<ParsedLine> =
        input.split('\n').filter_map(|l| parse_line(l)).collect();
    parsed_lines.sort();

    let mut acc = ResultAccumulator {
        current_guard: 0,
        sleep_mins: 0,
        guard_table: HashMap::new(),
    };

    for line in parsed_lines {
        match line.entry {
            LogEntry::ShiftBegin { id } => acc.current_guard = id,
            LogEntry::Sleep => acc.sleep_mins = line.timestamp.minute(),
            LogEntry::WakeUp => {
                let begin: usize = acc.sleep_mins as usize;
                let end: usize = line.timestamp.minute() as usize;
                let guard_entry = acc
                    .guard_table
                    .entry(acc.current_guard)
                    .or_insert(GuardEntry::new(acc.current_guard));
                for i in begin..end {
                    guard_entry.schedule[i] += 1;
                }
            }
        }
    }

    let id_mins: Vec<(u32, u32, u32, u32)> = acc
        .guard_table
        .iter()
        .map(|e| {
            (
                e.1.id,
                e.1.get_sleep_mins(),
                e.1.get_max_minute(),
                e.1.get_max_minute_value(),
            )
        })
        .collect();

    let max = id_mins.iter().max_by(|a, b| a.3.partial_cmp(&b.3).unwrap());
    println!(
        "id: {}, sleep mins: {}, max min: {}, max min val: {}",
        max.unwrap().0,
        max.unwrap().1,
        max.unwrap().2,
        max.unwrap().3
    );
    0
}

fn main() {
    solve_1(include_str!("../data/input.txt"));
}

#[test]
fn parse_line_test() {
    assert_eq!(
        Utc.ymd(1518, 08, 12).and_hms(00, 43, 00),
        parse_line("[1518-08-12 00:43] falls asleep")
            .unwrap()
            .timestamp
    );

    assert_eq!(
        LogEntry::Sleep,
        parse_line("[1518-08-12 00:43] falls asleep").unwrap().entry
    );
    assert_eq!(
        LogEntry::WakeUp,
        parse_line("[1518-08-12 00:43] wakes up").unwrap().entry
    );

    assert_eq!(
        LogEntry::ShiftBegin { id: 1069 },
        parse_line("[1518-08-12 00:43] Guard #1069 begins shift")
            .unwrap()
            .entry
    );
}
