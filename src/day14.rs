use std::collections::HashMap;
use std::ptr;
#[path = "utils/reader.rs"]
mod reader;

pub struct Polymer {
    template: Vec<char>,
    rules: HashMap<(char, char), char>,
}

impl Polymer {
    pub fn new(data: &str) -> Polymer {
        let mut template: Vec<char> = Vec::<char>::new();
        let mut rules: HashMap<(char, char), char> = HashMap::new();
        let mut reader = reader::BufReader::open(data.to_string()).unwrap();
        let mut buffer = String::new();
        while let Some(line) = reader.read_line(&mut buffer) {
            if template.is_empty() {
                template = line.unwrap().trim().chars().collect::<Vec<char>>();
                continue;
            }
            let line = line.unwrap().trim();
            if line.len() == 0 {
                continue;
            }
            let rule: Vec<&str> = line.split(" -> ").collect();
            let key = rule[0].chars().collect::<Vec<char>>();
            rules.insert(
                (key[0], key[1]),
                rule[1].chars().collect::<Vec<char>>()[0].clone(),
            );
        }
        Polymer { template, rules }
    }
    pub fn run(&mut self, steps: usize) -> usize {
        let mut freq: HashMap<char, usize> = HashMap::new();
        let mut pairs: HashMap<(char, char), usize> = HashMap::new();
        let mut newpairs: HashMap<(char, char), usize> = HashMap::new();
        for pos in 0..self.template.len() - 1 {
            let pair = (self.template[pos], self.template[pos + 1]);
            if let Some(p) = pairs.get_mut(&pair) {
                *p += 1;
            } else {
                pairs.insert(pair, 1);
            }
        }
        for c in &self.template {
            if let Some(p) = freq.get_mut(&c) {
                *p += 1;
            } else {
                freq.insert(*c, 1);
            }
        }
        for _step in 0..steps {
            // destroy old pairs
            for (k, v) in pairs.drain() {
                if let Some(p) = self.rules.get(&k) {
                    // make all products from old pair
                    if let Some(f) = freq.get_mut(p) {
                        *f += v;
                    } else {
                        freq.insert(*p, v);
                    }
                    // add all new left pairs
                    let mut np = (k.0, *p);
                    if let Some(p) = newpairs.get_mut(&np) {
                        *p += v;
                    } else {
                        newpairs.insert(np, v);
                    }
                    // add all new right pairs
                    np = (*p, k.1);
                    if let Some(p) = newpairs.get_mut(&np) {
                        *p += v;
                    } else {
                        newpairs.insert(np, v);
                    }
                }
            }
            // swap new and old pairs
            unsafe {
                ptr::swap(&mut pairs, &mut newpairs);
            }
        }
        let mut init = true;
        let mut high = 0;
        let mut low = 0;
        for v in freq.values() {
            if init {
                init = !init;
                high = *v;
                low = *v;
                continue;
            }
            if *v > high {
                high = *v;
            }
            if *v < low {
                low = *v;
            }
        }
        high - low
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let mut p = Polymer::new("data/test/input14");
        assert_eq!(1588, p.run(10));
        assert_eq!(2188189693529, p.run(40));
    }
}
