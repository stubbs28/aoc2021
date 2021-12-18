use std::collections::HashMap;
use std::collections::HashSet;
#[path = "utils/reader.rs"]
mod reader;

pub struct Cave {
    adjacency: HashMap<String, Vec<String>>,
}

impl Cave {
    pub fn new(data: &str) -> Cave {
        let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
        let mut reader = reader::BufReader::open(data.to_string()).unwrap();
        let mut buffer = String::new();
        while let Some(line) = reader.read_line(&mut buffer) {
            let edge: Vec<String> = line
                .unwrap()
                .trim()
                .split("-")
                .map(|x| x.to_string())
                .collect();
            let a = adjacency
                .entry(edge[0].clone())
                .or_insert(Vec::<String>::new());
            a.push(edge[1].clone());
            let b = adjacency
                .entry(edge[1].clone())
                .or_insert(Vec::<String>::new());
            b.push(edge[0].clone());
        }
        Cave { adjacency }
    }
    pub fn visit(&self, path: Vec<String>, visited: HashSet<String>, extravisit: bool) -> i32 {
        let mut count = 0;
        if let Some(current) = path.last() {
            if *current == "end".to_string() {
                return 1;
            }
            if let Some(adj) = self.adjacency.get(current) {
                for next in adj {
                    if next == "start" {
                        continue;
                    }
                    let mut p = path.clone();
                    let mut v = visited.clone();
                    p.push(next.to_string());
                    if *next == next.to_lowercase() {
                        match visited.get(next) {
                            None => {
                                v.insert(next.to_string());
                            }
                            Some(_e) => {
                                if extravisit {
                                    count += self.visit(p.clone(), v.clone(), !extravisit);
                                }
                                continue;
                            }
                        }
                    }
                    count += self.visit(p, v, extravisit);
                }
            }
        }
        count
    }
    pub fn traverse_extra(&self) -> i32 {
        let mut set = HashSet::new();
        set.insert("start".to_string());
        return self.visit(vec!["start".to_string()], set, true);
    }
    pub fn traverse(&self) -> i32 {
        let mut set = HashSet::new();
        set.insert("start".to_string());
        return self.visit(vec!["start".to_string()], set, false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let c = Cave::new("data/test/input12");
        assert_eq!(226, c.traverse());
    }
    #[test]
    fn part_two() {
        let c = Cave::new("data/test/input12");
        assert_eq!(3509, c.traverse_extra());
    }
}
