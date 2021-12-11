use std::io::prelude::*;
use std::fs::File;
use std::str::FromStr;

pub struct School {
    population: Vec::<i64>,
}

impl School {
    pub fn new(data: String) -> School {
        let mut p = Vec::<i64>::with_capacity(9);
        p.resize(9, 0);
        let mut file = File::open(data).expect("unable to open the file");
        let mut d = String::new();
        file.read_to_string(&mut d).expect("unable to read the file");
        let s: Vec::<&str> = d.trim().split(",")
            .filter(|s| !s.is_empty())
            .collect();
        for f in s {
            let pos = usize::from_str(f).unwrap();
            p[pos] += 1
        }
        School {
            population: p,
        }
    }

    pub fn reproduce(&mut self, days: i32) -> i64 {
        for d in 0..days {
            let eggs = self.population.remove(0);
            self.population.push(eggs);
            self.population[6] += eggs;
        }
        let mut c: i64 = 0;
        for f in &self.population {
            c += f;
        }
        c
    }

    pub fn print(&self) {
        println!("pop: {:#?}", self.population);
    }
}
