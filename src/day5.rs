#[path="utils/reader.rs"] mod reader;
use std::collections::HashMap;

pub struct VentMap {
    map: HashMap::<(i32,i32),i32>,
}

impl VentMap {
    pub fn new(vents: String) -> VentMap {
        let mut map = HashMap::new();
        let mut reader = reader::BufReader::open(vents).unwrap();
        let mut buffer = String::new();
        while let Some(line) = reader.read_line(&mut buffer) {
            let l: Vec::<&str> = line.unwrap().trim().split(" -> ").collect();
            let ps1: Vec::<&str> = l[0].split(",").collect();
            let ps2: Vec::<&str> = l[1].split(",").collect();
            let mut p1 = (
                ps1[0].parse::<i32>().unwrap(), 
                ps1[1].parse::<i32>().unwrap()
            );
            let mut p2 = (
                ps2[0].parse::<i32>().unwrap(), 
                ps2[1].parse::<i32>().unwrap()
            );
            if p1.0 != p2.0 && p1.1 != p2.1 {
                continue;
            }
            if p1.0 > p2.0 {
                let tmp = p1.0;
                p1.0 = p2.0;
                p2.0 = tmp;
            }
            if p1.1 > p2.1 {
                let tmp = p1.1;
                p1.1 = p2.1;
                p2.1 = tmp;
            }
            for x in p1.0..=p2.0 {
                for y in p1.1..=p2.1 {
                    if let Some(e) = map.get_mut(&(x, y)) {
                        *e += 1;
                    } else {
                        map.insert((x, y), 1);
                    }
                }
            }
        }
        VentMap{
            map: map,
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
