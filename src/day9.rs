#[path="utils/reader.rs"] mod reader;
use std::collections::HashMap;

pub struct Point {
    n: i8,
    height: i8,
}

impl Point {
    pub fn new() -> Point {
        Point {
            n: 10,
            height: 0,
        }
    }
    pub fn danger(&self) -> i64 {
        if self.height < self.n {
            return (self.height + 1 ).try_into().unwrap();
        } 
        0
    }
    pub fn add_height(&mut self, height: i8) {
        self.height = height;
    }
    pub fn add_neighbor(&mut self, neighbor: i8) {
        if neighbor < self.n {
            self.n = neighbor;
        }
    }
}

pub struct HeightMap {
    points: HashMap::<i32, Point>,
}

impl HeightMap {
    pub fn new(data: &str, rows: i32, cols: i32) -> HeightMap {
        let mut reader = reader::BufReader::open(data.to_string()).unwrap();
        let mut buffer = String::new();
        let mut points: HashMap::<i32, Point> = HashMap::new();
        let mut yoff = 0;
        while let Some(line) = reader.read_line(&mut buffer) {
            let mut xoff = 0;
            let l = line.unwrap().trim();
            for h in l.chars().map(|c| c.to_digit(10).unwrap()) {
                let cord = (yoff * cols) + xoff;
                let h: i8 = h.try_into().unwrap();
                if let Some(p) = points.get_mut(&cord) {
                    p.add_height(h);
                } else {
                    let mut p = Point::new();
                    p.add_height(h);
                    points.insert(cord, p);
                }
                // down
                if (yoff + 1) < rows {
                    let down = cord + cols;
                    if let Some(p) = points.get_mut(&down) {
                        p.add_neighbor(h);
                    } else {
                        let mut p = Point::new();
                        p.add_neighbor(h);
                        points.insert(cord + cols, p);
                    }
                } 
                // right
                if (xoff + 1) < cols {
                    let right = cord + 1;
                    if let Some(p) = points.get_mut(&right) {
                        p.add_neighbor(h);
                    } else {
                        let mut p = Point::new();
                        p.add_neighbor(h);
                        points.insert(right, p);
                    }
                } 
                // left
                if (xoff - 1) >= 0 {
                    let left = (yoff * cols) + (xoff - 1);
                    if let Some(p) = points.get_mut(&left) {
                        p.add_neighbor(h);
                    }
                }
                // up
                if (yoff - 1) >= 0 {
                    let up = ((yoff - 1) * cols) + xoff;
                    if let Some(p) = points.get_mut(&up) {
                        p.add_neighbor(h);
                    }
                }
                xoff += 1;
            };
            yoff += 1;
        }
        HeightMap {
            points: points,
        }
    }

    pub fn danger(&self) -> i64 {
        let mut sum: i64 = 0;
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
}

