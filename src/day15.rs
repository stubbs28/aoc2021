use std::collections::HashMap;
#[path = "utils/reader.rs"]
mod reader;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Vertex {
    point: (i32, i32),
    weight: u32,
}

impl Vertex {
    pub fn new(x: i32, y: i32, weight: u32) -> Vertex {
        Vertex {
            point: (x, y),
            weight,
        }
    }
    pub fn weight(&self) -> u32 {
        self.weight
    }
    pub fn neighbors(&self, maxx: i32, maxy: i32) -> Vec<i32> {
        let ns = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        ns.iter()
            .map(|x| (self.point.0 + x.0, self.point.1 + x.1))
            .filter(|x| x.0 >= 0 && x.0 < maxx && x.1 >= 0 && x.1 < maxy)
            .map(|x| (x.1 * maxy) + x.0)
            .collect()
    }
}

pub struct Chiton {
    maxx: i32,
    maxy: i32,
    graph: Vec<Vertex>,
}

impl Chiton {
    pub fn new(data: &str) -> Chiton {
        let mut graph = Vec::<Vertex>::new();
        let mut reader = reader::BufReader::open(data.to_string()).unwrap();
        let mut buffer = String::new();
        let mut x = 0;
        let mut y = 0;
        while let Some(line) = reader.read_line(&mut buffer) {
            x = -1;
            let line = line.unwrap().trim();
            graph.extend(line.chars().map(|w| {
                x += 1;
                Vertex::new(x, y, w.to_digit(10).unwrap())
            }));
            y += 1;
        }
        println!("maxx:{} maxy:{}", x + 1, y);
        Chiton {
            maxx: x + 1,
            maxy: y,
            graph,
        }
    }
    pub fn shortest_path(&self) -> u32 {
        let start = self.graph.first().unwrap();
        let target = self.graph.last().unwrap();
        let mut dist: HashMap<&Vertex, u32> = HashMap::new();
        let mut prev: HashMap<&Vertex, Option<&Vertex>> = HashMap::new();
        let mut q: Vec<&Vertex> = Vec::<&Vertex>::new();
        for v in &self.graph {
            dist.insert(v, u32::MAX);
            prev.insert(v, None);
            q.push(v);
        }
        if let Some(d) = dist.get_mut(&start) {
            *d = start.weight();
        }
        while !q.is_empty() {
            // set u to smallest dist[u]
            let mut u: (usize, Option<&Vertex>) = (0, None);
            for (pos, qq) in q.iter_mut().enumerate() {
                let distq = dist.get(qq).unwrap();
                match u.1 {
                    Some(uu) => {
                        if dist.get(uu).unwrap() > distq {
                            u = (pos, Some(qq));
                        }
                    }
                    None => u = (pos, Some(qq)),
                }
            }
            // remove u from q
            q.remove(u.0);
            let uu = u.1.unwrap();
            if uu == target {
                let mut sum = uu.weight();
                let mut path = Vec::<&Vertex>::new();
                let mut current = uu;
                loop {
                    let cur = prev.get(current).unwrap();
                    match cur {
                        Some(c) => {
                            println!("V:{:?}", c);
                            path.push(c);
                            sum += c.weight();
                            current = c;
                        }
                        None => break,
                    }
                }
                return sum - start.weight();
            }

            for pos in uu.neighbors(self.maxx, self.maxy) {
                let v = &self.graph[pos as usize];
                let alt = dist.get(uu).unwrap() + v.weight();
                let distv = dist.get_mut(v).unwrap();
                if alt < *distv {
                    *distv = alt;
                    *prev.get_mut(v).unwrap() = u.1;
                }
            }
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
        assert_eq!(40, c.shortest_path());
    }
}
