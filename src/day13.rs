use std::collections::HashSet;
#[path = "utils/reader.rs"]
mod reader;

pub struct Fold {
    dots: HashSet<(i32, i32)>,
    x: i32,
    y: i32,
}

impl Fold {
    pub fn new(x: i32, y: i32) -> Fold {
        Fold {
            dots: HashSet::new(),
            x,
            y,
        }
    }

    pub fn dots(&self) -> &HashSet<(i32, i32)> {
        &self.dots
    }

    pub fn fold_point(&mut self, p: (i32, i32)) {
        let mut np = p.clone();
        if self.x != -1 {
            if p.0 > self.x {
                np.0 = self.x - (p.0 - self.x);
            }
        } else if self.y != -1 {
            if p.1 > self.y {
                np.1 = self.y - (p.1 - self.y);
            }
        }
        self.dots.insert(np);
    }

    pub fn to_string(&self, canvas: (i32, i32)) -> String {
        let mut s = String::new();
        for y in 0..canvas.1 {
            for x in 0..canvas.0 {
                if let Some(_p) = self.dots.get(&(x, y)) {
                    s.push('#');
                    continue;
                }
                s.push('.');
            }
            s.push('\n');
        }
        s
    }
}

pub struct Paper {
    canvas: (i32, i32),
    dots: HashSet<(i32, i32)>,
    folds: Vec<Fold>,
}

impl Paper {
    pub fn new(data: &str) -> Paper {
        let mut dots: HashSet<(i32, i32)> = HashSet::new();
        let mut folds = Vec::<Fold>::new();
        let mut reader = reader::BufReader::open(data.to_string()).unwrap();
        let mut buffer = String::new();
        let mut canvas = (-1, -1);
        while let Some(line) = reader.read_line(&mut buffer) {
            let line = line.unwrap().trim();
            if line.len() == 0 {
                continue;
            }
            if let Some(fold) = line.strip_prefix("fold along ") {
                if let Some(x) = fold.strip_prefix("x=") {
                    let x = x.parse::<i32>().unwrap();
                    canvas.0 = x;
                    folds.push(Fold::new(x, -1));
                    continue;
                }
                if let Some(y) = fold.strip_prefix("y=") {
                    let y = y.parse::<i32>().unwrap();
                    canvas.1 = y;
                    folds.push(Fold::new(-1, y));
                    continue;
                }
                panic!("we missed a fold");
            }
            let dot: Vec<i32> = line
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            dots.insert((dot[0], dot[1]));
        }
        Paper {
            canvas,
            dots,
            folds,
        }
    }

    pub fn to_string(&self) -> String {
        self.folds.last().unwrap().to_string(self.canvas)
    }

    pub fn count(&self, fold: usize) -> usize {
        self.folds[fold].dots().len()
    }

    pub fn fold(&mut self) {
        let mut dots = &self.dots;
        for f in &mut self.folds {
            for d in dots {
                f.fold_point(d.clone());
            }
            dots = f.dots();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let mut p = Paper::new("data/test/input13");
        p.fold();
        assert_eq!(17, p.count(0));
    }
}
