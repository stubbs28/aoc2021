#[path = "utils/reader.rs"]
mod reader;
pub struct Scanner {
    source: String,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner { source: source }
    }

    pub fn scan(&self, window: usize) -> i32 {
        let mut reader = reader::BufReader::open(&self.source).unwrap();
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
