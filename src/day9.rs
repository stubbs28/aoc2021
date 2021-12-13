#[path="utils/reader.rs"] mod reader;
use std::collections::HashMap;


pub struct Point {
    n: i8,
    height: i8,
    visited: bool,
}

impl Point {
    pub fn new() -> Point {
        Point {
            n: 10,
            height: 0,
            visited: false,
        }
    }
    pub fn height(&self) -> i8 {
        self.height
    }
    pub fn danger(&self) -> i32 {
        if self.height < self.n {
            return (self.height + 1).try_into().unwrap();
        } 
        0
    }
    pub fn visited(&self) -> bool {
        self.visited
    }
    pub fn visit(&mut self){
        self.visited = true;
    }
    pub fn add_height(&mut self, height: i8) {
        if height == 9 {
            self.visited = true;
        }
        self.height = height;
    }
    pub fn add_neighbor(&mut self, neighbor: i8) {
        if neighbor < self.n {
            self.n = neighbor;
        }
    }
}

pub struct HeightMap {
    rows: i32,
    cols: i32,
    points: HashMap::<(i32, i32), Point>,
}

impl HeightMap {
    pub fn new(data: &str, rows: i32, cols: i32) -> HeightMap {
        let mut reader = reader::BufReader::open(data.to_string()).unwrap();
        let mut buffer = String::new();
        let mut points: HashMap::<(i32, i32), Point> = HashMap::new();
        let mut y = 0;
        while let Some(line) = reader.read_line(&mut buffer) {
            let mut x = 0;
            let l = line.unwrap().trim();
            for h in l.chars().map(|c| c.to_digit(10).unwrap()) {
                let cord = (x, y);
                let h: i8 = h.try_into().unwrap();
                if let Some(p) = points.get_mut(&cord) {
                    p.add_height(h);
                } else {
                    let mut p = Point::new();
                    p.add_height(h);
                    points.insert(cord, p);
                }
                // down
                if (y + 1) < rows {
                    let down = (x, y + 1);
                    if let Some(p) = points.get_mut(&down) {
                        p.add_neighbor(h);
                    } else {
                        let mut p = Point::new();
                        p.add_neighbor(h);
                        points.insert(down, p);
                    }
                } 
                // right
                if (x + 1) < cols {
                    let right = (x + 1, y);
                    if let Some(p) = points.get_mut(&right) {
                        p.add_neighbor(h);
                    } else {
                        let mut p = Point::new();
                        p.add_neighbor(h);
                        points.insert(right, p);
                    }
                } 
                // left
                if (x - 1) >= 0 {
                    let left = (x - 1, y);
                    if let Some(p) = points.get_mut(&left) {
                        p.add_neighbor(h);
                    }
                }
                // up
                if (y - 1) >= 0 {
                    let up = (x, y - 1);
                    if let Some(p) = points.get_mut(&up) {
                        p.add_neighbor(h);
                    }
                }
                x += 1;
            };
            y += 1;
        }
        HeightMap {
            rows: rows,
            cols: cols,
            points: points,
        }
    }

    fn basin(&mut self, h: i8, cord: (i32, i32)) -> i32{
        let mut count = 0;
        let mut h: i8 = -1;
        match self.points.get_mut(&cord) {
            Some(p) => {
                if !p.visited() && p.height() > h {
                    p.visit();
                    h = p.height();
                    count += 1;
                } else {
                    return count;
                }
            }
            None => {
                return count;
            }
        }
        count += self.basin(h, (cord.0, cord.1 + 1));
        count += self.basin(h, (cord.0, cord.1 - 1));
        count += self.basin(h, (cord.0 + 1, cord.1));
        count += self.basin(h, (cord.0 - 1, cord.1));
        count
    }

    pub fn basins(&mut self) -> i32 {
        let mut b = Vec::<i32>::new();
        for x in 0..self.cols {
            for y in 0..self.rows {
                let c = (x, y);
                match self.points.get_mut(&c) {
                    Some(p) => {
                        if p.danger() == 0 { 
                            continue; 
                        }
                    },
                    None => continue,
                }
                b.push(self.basin(-1, c));
            }
        }
        b.sort();
        let mut danger = Option::<i32>::None;
        for i in 0..3 {
            if let Some(d) = b.pop() {
                match danger.as_mut() {
                    Some(v) => *v *= d,
                    None => danger = Some(d),
                }
            }
        }
        match danger {
            Some(v) => v,
            None => 0,
        }
    }

    pub fn danger(&self) -> i32 {
        let mut sum: i32 = 0;
        for p in self.points.values() {
            sum += p.danger();
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let h = HeightMap::new("data/input9_test", 5, 10);
        assert_eq!(15, h.danger());
    }

    #[test]
    fn part_two() {
        let mut h = HeightMap::new("data/input9_test", 5, 10);
        assert_eq!(1134, h.basins());
    }
}

