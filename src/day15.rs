use std::collections::{HashMap, HashSet};
#[path = "utils/point.rs"]
mod point;
#[path = "utils/reader.rs"]
mod reader;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Vertex {
    point: point::Point,
    prev: Option<point::Point>,
    weight: u32,
    dist: u32,
    done: bool,
}

impl Vertex {
    pub fn new(point: point::Point, weight: u32) -> Vertex {
        Vertex {
            point,
            prev: None,
            weight,
            dist: u32::MAX,
            done: false,
        }
    }
    pub fn new_relative(&self, point: point::Point, inc: u32) -> Vertex {
        let mut w = self.weight + inc;
        if w > 9 {
            w = (w % 10) + 1;
        }
        Vertex::new(point, w)
    }
    pub fn start(&mut self) {
        self.dist = 0;
    }
    pub fn update_distance(&mut self, prev: point::Point, distu: u32) {
        let alt = distu + self.weight;
        if alt < self.dist {
            self.dist = alt;
            self.prev = Some(prev);
        }
    }
    pub fn distance(&self) -> u32 {
        self.dist
    }
    pub fn weight(&self) -> u32 {
        self.weight
    }
    pub fn mark_done(&mut self) {
        self.done = true;
    }
    pub fn is_done(&self) -> bool {
        self.done
    }
    pub fn prev(&self) -> Option<point::Point> {
        self.prev
    }
    pub fn neighbors(&self, max: &point::Point) -> Vec<point::Point> {
        let ns = vec![
            point::Point::new(-1, 0),
            point::Point::new(1, 0),
            point::Point::new(0, -1),
            point::Point::new(0, 1),
        ];
        ns.into_iter()
            .map(|x| x + self.point)
            .filter(|x| x.in_bounds(&max))
            .collect()
    }
}

pub struct Chiton {
    max: point::Point,
    graph: HashMap<point::Point, Vertex>,
}

impl Chiton {
    pub fn new(data: &str) -> Chiton {
        let mut graph = HashMap::<point::Point, Vertex>::new();
        let mut reader = reader::BufReader::open(data.to_string()).unwrap();
        let mut buffer = String::new();
        let mut cur = point::Point::new(0, 0);
        while let Some(line) = reader.read_line(&mut buffer) {
            let line = line.unwrap().trim();
            cur.set_x(0);
            graph.extend(line.chars().map(|w| {
                let p = cur.clone();
                cur.inc_x();
                (p, Vertex::new(p, w.to_digit(10).unwrap()))
            }));
            cur.inc_y()
        }
        Chiton { max: cur, graph }
    }
    pub fn shortest_path(&self, scaler: i32) -> u32 {
        let start = point::Point::new(0, 0);
        let max = self.max.scale(scaler);
        let target = max.step(-1);
        let mut graph = self.graph.clone();
        let mut q: Vec<point::Point> = Vec::<point::Point>::new();
        let mut inq: HashSet<point::Point> = HashSet::new();
        graph.get_mut(&start).unwrap().start();
        q.push(start);
        while !q.is_empty() {
            let u = q.pop().unwrap();
            inq.remove(&u);
            graph.get_mut(&u).unwrap().mark_done();
            if u == target {
                let mut path = Vec::<point::Point>::new();
                let mut cur = u;
                let mut sum = 0;
                while cur != start {
                    path.push(cur);
                    let c = graph.get(&cur).unwrap();
                    sum += c.weight();
                    cur = c.prev().unwrap();
                }
                return sum;
            }
            let neighbors: Vec<point::Point>;
            let distu: u32;
            match graph.get(&u) {
                Some(su) => {
                    neighbors = su.neighbors(&max);
                    distu = su.distance();
                }
                None => panic!("u not in graph"),
            }
            for pos in neighbors {
                if !graph.contains_key(&pos) {
                    let div = pos / self.max;
                    let inc = (div.x() + div.y()) as u32;
                    graph.insert(
                        pos,
                        graph.get(&(pos % self.max)).unwrap().new_relative(pos, inc),
                    );
                }
                let v = graph.get_mut(&pos).unwrap();
                if v.is_done() {
                    continue;
                }
                v.update_distance(u, distu);
                if !inq.contains(&pos) {
                    q.push(pos);
                    inq.insert(pos);
                }
            }
            q.sort_unstable_by(|a, b| {
                let dista = graph.get(a).unwrap().distance();
                let distb = graph.get(b).unwrap().distance();
                distb.cmp(&dista)
            });
        }
        u32::MAX
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let c = Chiton::new("data/test/input15");
        assert_eq!(40, c.shortest_path(1));
    }
    #[test]
    fn part_two() {
        let c = Chiton::new("data/test/input15");
        assert_eq!(315, c.shortest_path(5));
    }
}
