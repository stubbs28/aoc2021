use std::collections::HashMap;
use std::collections::HashSet;
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
        let mut prev: HashMap<&Vertex, &Vertex> = HashMap::new();
        let mut q: Vec<&Vertex> = Vec::<&Vertex>::new();
        let mut inq: HashSet<&Vertex> = HashSet::new();
        let mut done: HashSet<&Vertex> = HashSet::new();
        dist.insert(&start, 0);
        q.push(&start);
        while !q.is_empty() {
            // set u to smallest dist[u]
            let u = q.pop().unwrap();
            // remove u from q
            inq.remove(&u);
            done.insert(u);
            // we got to the target, we can stop.
            if *u == *target {
                let mut sum = u.weight();
                let mut path = Vec::<&Vertex>::new();
                let mut current = u;
                loop {
                    match prev.get(&current) {
                        Some(pc) => {
                            path.push(pc);
                            sum += pc.weight();
                            current = pc;
                        }
                        None => break,
                    }
                }
                return sum - start.weight();
            }

            for pos in u.neighbors(self.maxx, self.maxy) {
                let v = &self.graph[pos as usize];
                if done.contains(&v) {
                    continue;
                }
                let mut alt = v.weight();
                match dist.get(u) {
                    Some(distu) => alt += *distu,
                    None => {
                        dist.insert(u, u32::MAX);
                        alt += u32::MAX;
                    }
                }
                let mut distv = u32::MAX;
                match dist.get(v) {
                    Some(dv) => distv = *dv,
                    None => {
                        dist.insert(v, distv);
                    }
                }
                if alt < distv {
                    match dist.get_mut(&v) {
                        Some(dv) => *dv = alt,
                        None => {
                            dist.insert(&v, alt);
                        }
                    }
                    match prev.get_mut(&v) {
                        Some(pv) => *pv = u,
                        None => {
                            prev.insert(&v, u);
                        }
                    }
                }
                // NB: yes I know I can just binary search and do a sorted insert
                // but I'm tired.
                if !inq.contains(&v) {
                    inq.insert(v);
                    q.push(v);
                }
            }
            // NB: look... i'm sorry
            q.sort_unstable_by(|a, b| {
                let dista = dist.get(a).unwrap();
                let distb = dist.get(b).unwrap();
                distb.cmp(dista)
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
        assert_eq!(40, c.shortest_path());
    }
}
