struct Scanner {
    source: String
}

impl Scanner {
    fn new(source: String) -> Scanner {
        Scanner {source: source}
    }

    fn scan(&self, window: usize) -> i32 {
        let mut reader = my_reader::BufReader::open(&self.source).unwrap();
        let mut buffer = String::new();
        let mut points = Vec::new();
        let mut close = 0;
        let mut far = 0;
        let mut count = 0;
        while let Some(line) = reader.read_line(&mut buffer) {
            let point = line.unwrap().trim().parse::<i32>().unwrap();
            points.push(point);
            if points.len() <= window {
                close += point;
            }
            if points.len() > 1 {
                far += point;
            }
            if points.len() > window && far > close {
                count += 1;
            }
            if points.len() > window {
                close -= points[0];
                far -= points[1];
                close += points[window];
                points.remove(0);
            }
        }
        count
    }
}

fn main() {
    println!("==Day One==");
    let s = Scanner::new("data/input.txt".to_string());
    println!("part one: {}", s.scan(1));
    println!("part two: {}", s.scan(3));
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

