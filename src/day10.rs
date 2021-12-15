use itertools::Itertools;
use std::collections::HashMap;
#[path = "utils/reader.rs"]
mod reader;

pub struct Entry {
    //line: String,
    cscore: i32,
    fscore: i64,
}

impl Entry {
    pub fn new(data: &str) -> Entry {
        let mut stack = Vec::<char>::new();
        let cscoremap = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
        let fscoremap = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
        let mut cscore = 0;
        let mut fscore = 0;
        for c in data.chars() {
            match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),
                _ => match stack.pop() {
                    Some(expected) => {
                        if c != expected {
                            cscore = *cscoremap.get(&c).unwrap();
                            break;
                        }
                    }
                    _ => panic!("unexpected close"),
                },
            }
        }
        while let Some(c) = stack.pop() {
            fscore *= 5;
            fscore += fscoremap.get(&c).unwrap();
        }
        Entry {
            cscore: cscore,
            fscore: fscore,
        }
    }
    pub fn cscore(&self) -> i32 {
        self.cscore
    }
    pub fn fscore(&self) -> i64 {
        self.fscore
    }
}

pub struct Navigation {
    lines: Vec<Entry>,
}

impl Navigation {
    pub fn new(data: &str) -> Navigation {
        let mut reader = reader::BufReader::open(data.to_string()).unwrap();
        let mut buffer = String::new();
        let mut lines = Vec::<Entry>::new();
        while let Some(line) = reader.read_line(&mut buffer) {
            lines.push(Entry::new(line.unwrap().trim()));
        }
        Navigation { lines: lines }
    }
    pub fn corrupted_score(&self) -> i32 {
        self.lines.iter().map(|s| s.cscore()).sum()
    }
    pub fn fixed_score(&self) -> i64 {
        let scores: Vec<i64> = self
            .lines
            .iter()
            .filter(|s| s.cscore() == 0)
            .map(|s| s.fscore())
            .sorted()
            .collect();
        let mid: usize = scores.len() / 2;
        scores[mid]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let n = Navigation::new("data/test/input10");
        assert_eq!(26397, n.corrupted_score());
    }

    #[test]
    fn part_two() {
        let n = Navigation::new("data/test/input10");
        assert_eq!(288957, n.fixed_score());
    }
}
