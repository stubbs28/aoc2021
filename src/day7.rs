use std::io::prelude::*;
use std::fs::File;
use std::str::FromStr;
use std::collections::HashMap;

pub struct Crabs {
    map: HashMap::<i32, i32>,
}

impl Crabs {
    pub fn new(data: String) -> Crabs {
        let mut m = HashMap::new();
        let mut file = File::open(data).expect("unable to open the file");
        let mut d = String::new();
        file.read_to_string(&mut d).expect("unable to read the file");
        let s: Vec::<&str> = d.trim().split(",")
            .filter(|s| !s.is_empty())
            .collect();
        for f in s {
            let pos = f.parse::<i32>().unwrap();
            if let Some(e) = m.get_mut(&pos) {
                *e += 1;
            } else {
                m.insert(pos, 1);
            }
        }
        Crabs {
            map: m,
        }
    }

    pub fn smallest_align_cost(&self) -> i32 {
        let mut c = -1;
        for target in self.map.keys() {
            let ac = self.align_cost(target);
            if c == -1 || ac < c {
                c = ac;
            }
        }
        c
    }

    pub fn align_cost(&self, target: &i32) -> i32 {
        let mut c = 0;
        for (key, val) in self.map.iter() {
            if *key == *target {
                continue;
            }
            let mut diff = *key - *target;
            if diff < 0 {
                diff *= -1;
            }
            c += diff * val;
        }
        c
    }

    pub fn print(&self) {
        println!("pop: {:#?}", self.map);
    }
}
