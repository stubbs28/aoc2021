#[path = "utils/reader.rs"]
mod reader;
use std::collections::HashMap;

pub struct VentMap {
    map: HashMap<(i32, i32), i32>,
    p1: Vec<(i32, i32)>,
    p2: Vec<(i32, i32)>,
}

impl VentMap {
    pub fn new(vents: String) -> VentMap {
        let mut p1 = Vec::<(i32, i32)>::new();
        let mut p2 = Vec::<(i32, i32)>::new();
        let mut reader = reader::BufReader::open(vents).unwrap();
        let mut buffer = String::new();
        while let Some(line) = reader.read_line(&mut buffer) {
            let l: Vec<&str> = line.unwrap().trim().split(" -> ").collect();
            let ps1: Vec<&str> = l[0].split(",").collect();
            let ps2: Vec<&str> = l[1].split(",").collect();
            p1.push((
                ps1[0].parse::<i32>().unwrap(),
                ps1[1].parse::<i32>().unwrap(),
            ));
            p2.push((
                ps2[0].parse::<i32>().unwrap(),
                ps2[1].parse::<i32>().unwrap(),
            ));
        }
        VentMap {
            map: HashMap::new(),
            p1: p1,
            p2: p2,
        }
    }

    pub fn map_horz_vert(&mut self) {
        for i in 0..self.p1.len() {
            let p1 = self.p1[i];
            let p2 = self.p2[i];
            if p1.0 != p2.0 && p1.1 != p2.1 {
                continue;
            }
            let mut xdif = p1.0 - p2.0;
            let mut xdir = -1;
            if xdif < 0 {
                xdif *= -1;
                xdir *= -1;
            }
            let mut ydif = p1.1 - p2.1;
            let mut ydir = -1;
            if ydif < 0 {
                ydif *= -1;
                ydir *= -1;
            }
            for x in 0..=xdif {
                for y in 0..=ydif {
                    let pos = (p1.0 + (x * xdir), p1.1 + (y * ydir));
                    if let Some(e) = self.map.get_mut(&pos) {
                        *e += 1;
                    } else {
                        self.map.insert(pos, 1);
                    }
                }
            }
        }
    }

    pub fn map_diag(&mut self) {
        for i in 0..self.p1.len() {
            let p1 = self.p1[i];
            let p2 = self.p2[i];
            if p1.0 == p2.0 || p1.1 == p2.1 {
                continue;
            }
            let mut xdif = p1.0 - p2.0;
            let mut xdir = -1;
            if xdif < 0 {
                xdif *= -1;
                xdir *= -1;
            }
            let ydif = p1.1 - p2.1;
            let mut ydir = -1;
            if ydif < 0 {
                ydir *= -1;
            }
            for off in 0..=xdif {
                let pos = (p1.0 + (off * xdir), p1.1 + (off * ydir));
                if let Some(e) = self.map.get_mut(&pos) {
                    *e += 1;
                } else {
                    self.map.insert(pos, 1);
                }
            }
        }
    }

    pub fn danger_score(&self) -> i32 {
        let mut danger = 0;
        for val in self.map.values() {
            if *val >= 2 {
                danger += 1
            }
        }
        danger
    }
}
