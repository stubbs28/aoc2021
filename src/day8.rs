#[path="utils/reader.rs"] mod reader;

pub struct Note {
    signals: Vec::<String>,
    segments: Vec::<String>,
    displays: Vec::<i32>,
}

impl Note {
    pub fn new(input: &Vec::<&str>) -> Note {
        let mut signals = Vec::<String>::with_capacity(10);
        let mut segments = Vec::<String>::with_capacity(4);
        let mut displays = Vec::<i32>::with_capacity(4);
        for i in input {
            if signals.len() < 10 {
                signals.push(i.to_string());
                continue;
            }
            segments.push(i.to_string());
            match i.len() {
                2=> displays.push(1), // 1: cf
                3=> displays.push(7), // 7: adf
                4=> displays.push(4), // 4: bcdf
                7=> displays.push(8), // 8: abcdefg
                _=> displays.push(-1),
            };
        }
        Note {
            signals: signals,
            segments: segments,
            displays: displays,
        }
    }
    pub fn known_count(&self) -> i32 {
        let mut count = 0;
        for i in &self.displays {
            if *i != -1 {
                count += 1;
            }
        }
        count
    }
}

pub struct Notes {
    notes: Vec::<Note>,
}

impl Notes {
    pub fn new(data: &str) -> Notes {
        let mut reader = reader::BufReader::open(data.to_string()).unwrap();
        let mut buffer = String::new();
        let mut notes = Vec::<Note>::new();
        while let Some(line) = reader.read_line(&mut buffer) {
            let l: Vec::<&str> = line.unwrap().trim().split(" ").collect();
            notes.push(Note::new(&l));
        }
        Notes {
            notes: notes,
        }
    }
    pub fn known_count(&self) -> i32 {
        let mut count = 0;
        for n in &self.notes {
            count += n.known_count();
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let notes = Notes::new("data/input8_test");
        assert_eq!(26, notes.known_count());
    }
}
