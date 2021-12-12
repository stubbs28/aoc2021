use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

pub struct Crabs {
    map: HashMap::<i32, i32>,
    range: (i32, i32),
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
        let mut r: (i32, i32) = (-1, -1);
        for f in s {
            let pos = f.parse::<i32>().unwrap();
            if r.0 == -1 || r.0 > pos {
                r.0 = pos;
            }
            if r.1 < pos {
                r.1 = pos;
            }
            if let Some(e) = m.get_mut(&pos) {
                *e += 1;
            } else {
                m.insert(pos, 1);
            }
        }
        Crabs {
            map: m,
            range: r,
        }
    }

    pub fn human_align_cost(&self, ) -> i32 {
        let mut c = -1;
        for target in self.map.keys() {
            let mut ac = 0;
            for (key, val) in self.map.iter() {
                if *key == *target {
                    continue;
                }
                let mut diff = *key - *target;
                if diff < 0 {
                    diff *= -1;
                }
                ac += diff * val;
            }
            if c == -1 || ac < c {
                c = ac;
            }
        }
        c
    }

    pub fn crab_align_cost(&self, ) -> i32 {
        let mut c = -1;
        for target in self.range.0..self.range.1 {
            let mut ac = 0;
            for (key, val) in self.map.iter() {
                if *key == target {
                    continue;
                }
                let mut diff = *key - target;
                if diff < 0 {
                    diff *= -1;
                }
                for d in 0..diff {
                    ac += (d + 1) * val;
                }
            }
            if c == -1 || ac < c {
                c = ac;
            }
        }
        c
    }
}
