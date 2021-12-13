#[path="utils/reader.rs"] mod reader;
use std::collections::HashMap;
use std::collections::HashSet;
use itertools::Itertools;

// UNIQUE
// 1: cf
// 7: acf
// 4: bcdf
// 8: abcdefg

// 2: adgce | (2 - 1) == 4 | (2 - 4) == 3 | (2 - 7) == 3
// 3: adgcf | (3 - 1) == 3 | (3 - 4) == 2 | (3 - 7) == 2
// 5: adgfb | (5 - 1) == 4 | (5 - 4) == 2 | (5 - 7) == 3
//
// 0: abcfge | (0 - 1) == 4 | (0 - 4) == 3 | (0 - 7) == 3
// 6: abdefg | (6 - 1) == 5 | (6 - 4) == 3 | (6 - 7) == 4
// 9: abcfgd | (9 - 1) == 4 | (9 - 4) == 2 | (9 - 7) == 3

macro_rules! known_digit {
    ($i:expr) => {
        match $i {
            2=> Some(1),
            3=> Some(7),
            4=> Some(4),
            7=> Some(8),
            _=> None
        }
    };
}

pub struct Display {
    encoding: Vec::<String>, // ie. encoding[0] == "abcefg"
    wireing: HashMap::<String, usize>, // ie. {"abcefg": 0}
    segments: HashMap::<usize, HashSet::<char>>,
    unknown: Vec::<String>,
    digits: Vec::<String>, 
}

impl Display {
    pub fn new(signals: &Vec::<&str>, segments: &Vec::<&str>) -> Display {
        let mut encoding = Vec::<String>::with_capacity(10);
        encoding.resize(10, "".to_string());
        let mut wireing = HashMap::new();
        let mut segs = HashMap::new();
        let mut unknown = Vec::<String>::with_capacity(6);
        let mut digits = Vec::<String>::with_capacity(4);
        for sig in signals {
            let s = sig.chars().sorted().collect::<String>();
            if let Some(d) = known_digit!(s.len()) {
                encoding[d] = s.clone();
                wireing.insert(s.clone(), d);
                segs.insert(d, s.chars().collect());
            } else {
                unknown.push(s.clone());
            }
        }
        for seg in segments {
            digits.push(seg.chars().sorted().collect::<String>());
        }
        for i in 0..encoding.len() {
            if let Some(map) = wireing.get(&encoding[i]) {
                if i != *map {
                    encoding.swap(i, *map);
                }
            }
        }
        Display {
            encoding: encoding,
            wireing: wireing,
            segments: segs,
            unknown: unknown,
            digits: digits,
        }
    }

    pub fn known_count(&self) -> i32 {
        let mut count = 0;
        for e in &self.digits {
            if self.wireing.get(e) != None {
                count += 1;
            } 
        }
        count
    }

    // returns how many characters in a are not in b.
    fn sub(&self, a: usize, b: usize) -> i32 {
        let mut count = 0;
        for c in self.unknown[a].chars() {
            if !self.segments[&b].contains(&c) {
                count += 1;
            }
        }
        return count;
    }

    fn sub_target(&self, u: usize, t: (i32, i32)) -> bool {
        self.sub(u, 1) == t.0 && self.sub(u, 4) == t.1
    }

    fn map(&mut self, a: usize, b: usize) {
        self.encoding[a] = self.unknown[b].clone();
        self.wireing.insert(self.unknown[b].clone(), a);
    }

    pub fn map_signals(&mut self) {
        let mut found = HashSet::<usize>::new();
        for pos in 0..6 {
            match self.unknown[pos].len() {
                5 => {
                    if !found.contains(&2) && self.sub_target(pos, (4, 3)) {
                        self.map(2, pos);
                        found.insert(2);
                    } else if !found.contains(&3) && self.sub_target(pos, (3, 2)) {
                        self.map(3, pos);
                        found.insert(3);
                    } else if !found.contains(&5) && self.sub_target(pos, (4, 2)) {
                        self.map(5, pos);
                        found.insert(5);
                    }
                },
                6 => {
                    if !found.contains(&0) && self.sub_target(pos, (4, 3)) {
                        self.map(0, pos);
                        found.insert(0);
                    } else if !found.contains(&6) && self.sub_target(pos, (5, 3)) {
                        self.map(6, pos);
                        found.insert(6);
                    } else if !found.contains(&9) && self.sub_target(pos, (4, 2)) {
                        self.map(9, pos);
                        found.insert(9);
                    }
                },
                _ => panic!("this should be known"),
            }
        }
    }

    pub fn output(&self) -> Option<i32> {
        let mut out = 0;
        let mut off = 1000;
        for e in &self.digits {
            if let Some(d) = self.wireing.get(e) {
                out += d * off;
            } else {
                return None
            }
            off /= 10;
        }
        Some(out.try_into().unwrap())
    }
}

pub struct Notes {
    notes: Vec::<Display>,
}

impl Notes {
    pub fn new(data: &str) -> Notes {
        let mut reader = reader::BufReader::open(data.to_string()).unwrap();
        let mut buffer = String::new();
        let mut notes = Vec::<Display>::new();
        while let Some(line) = reader.read_line(&mut buffer) {
            let i: Vec::<&str> = line.unwrap().trim().split(" | ").collect();
            let sig: Vec::<&str> = i[0].split(" ").collect();
            let seg: Vec::<&str> = i[1].split(" ").collect();
            notes.push(Display::new(&sig, &seg));
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
    pub fn map_signals(&mut self) {
        for n in &mut self.notes {
            n.map_signals();
        }
    }

    pub fn output_sum(&self) -> i32 {
        let mut count = 0;
        for n in &self.notes {
            if let Some(out) = n.output() {
                count += out;
            }
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

    #[test]
    fn part_two() {
        let mut notes = Notes::new("data/input8_test");
        notes.map_signals();
        assert_eq!(61229, notes.output_sum());
    }
}

