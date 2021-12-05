struct Password {
    value: String,
    lower: i32,
    upper: i32,
    policy: char,
}

impl Password {
    fn new(entry: String) -> Password {
        let e: Vec<&str> = entry
            .split(" ")
            .filter(|s| !s.is_empty())
            .collect();
        let b: Vec<&str> = e[0]
            .split("-")
            .filter(|s| !s.is_empty())
            .collect();
        Password {
            value: e[2].to_string(),
            lower: b[0].parse::<i32>().unwrap(),
            upper: b[1].parse::<i32>().unwrap(),
            policy: e[1].chars().nth(0).unwrap(),
        }
    }
    fn valid(&self) -> bool {
        let mut count = 0;
        for (_i, c) in self.value.chars().enumerate() {
            if c == self.policy {
                count += 1
            }
            if count > self.upper {
                return false;
            }
        }
        count >= self.lower
    }
}

struct Scanner {
    source: String
}

impl Scanner {
    fn new(source: String) -> Scanner {
        Scanner {source: source}
    }

    fn count_valid(&self) -> i32 {
        let mut reader = my_reader::BufReader::open(&self.source).unwrap();
        let mut buffer = String::new();
        let mut count = 0;
        while let Some(line) = reader.read_line(&mut buffer) {
            let password = Password::new(line.unwrap().trim().to_string());
            if password.valid() {
                count += 1
            }
        }
        count
    }
}

fn main() {
    println!("==Day Two==");
    let s = Scanner::new("data/input.txt".to_string());
    println!("part one: {}", s.count_valid());
}

mod my_reader {
    use std::{
        fs::File,
        io::{self, prelude::*},
    };

    pub struct BufReader {
        reader: io::BufReader<File>,
    }

    impl BufReader {
        pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);

            Ok(Self { reader })
        }

        pub fn read_line<'buf>(
            &mut self,
            buffer: &'buf mut String,
        ) -> Option<io::Result<&'buf mut String>> {
            buffer.clear();

            self.reader
                .read_line(buffer)
                .map(|u| if u == 0 { None } else { Some(buffer) })
                .transpose()
        }
    }
}

