use std::collections::HashMap;
#[path = "utils/reader.rs"]
mod reader;

pub struct Polymer {
    template: String,
    rules: HashMap<String, char>,
}

impl Polymer {
    pub fn new(data: &str) -> Polymer {
        let mut template = String::new();
        let mut rules: HashMap<String, char> = HashMap::new();
        let mut reader = reader::BufReader::open(data.to_string()).unwrap();
        let mut buffer = String::new();
        while let Some(line) = reader.read_line(&mut buffer) {
            if template.is_empty() {
                template = line.unwrap().trim().to_string();
                continue;
            }
            let line = line.unwrap().trim();
            if line.len() == 0 {
                continue;
            }
            let rule: Vec<String> = line.split(" -> ").map(|x| x.to_string()).collect();
            rules.insert(
                rule[0].clone(),
                rule[1].chars().collect::<Vec<char>>()[0].clone(),
            );
        }
        Polymer { template, rules }
    }
    pub fn step(&self, polymer: String) -> String {
        let mut product = String::with_capacity((polymer.len() * 2) - 1);
        let mut chain = polymer.chars();
        let mut a = chain.next().unwrap();
        while let Some(b) = chain.next() {
            product.push(a);
            let s: String = [a, b].iter().collect();
            if let Some(p) = self.rules.get(&s) {
                product.push(p.clone());
            }
            a = b;
        }
        product.push(a);
        product
    }
    pub fn run(&self, steps: i32) -> i32 {
        let mut s = self.template.clone();
        for _ in 0..steps {
            s = self.step(s);
        }
        let mut h: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            if let Some(x) = h.get_mut(&c) {
                *x += 1;
            } else {
                h.insert(c, 1);
            }
        }
        let mut high = '0';
        let mut low = '0';
        for (k, v) in h.iter() {
            if high == '0' && low == '0' {
                high = *k;
                low = *k;
                continue;
            }
            if v > h.get(&high).unwrap() {
                high = *k;
            }
            if v < h.get(&low).unwrap() {
                low = *k;
            }
        }
        h.get(&high).unwrap() - h.get(&low).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let p = Polymer::new("data/test/input14");
        assert_eq!(1588, p.run(10));
    }
}
