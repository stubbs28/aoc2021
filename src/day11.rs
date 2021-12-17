#[path = "utils/reader.rs"]
mod reader;

pub struct Octo {
    coord: (i16, i16),
    start: i8,
    energy: i8,
    flashed: i32,
}

impl Octo {
    pub fn new(coord: (i16, i16), energy: i8) -> Octo {
        Octo {
            coord,
            start: energy,
            energy,
            flashed: -1,
        }
    }
    pub fn reset(&mut self) {
        self.energy = self.start;
        self.flashed = -1;
    }
    pub fn coord(&self) -> (i16, i16) {
        self.coord
    }
    pub fn step(&mut self, step: i32) -> bool {
        if step > self.flashed {
            self.energy += 1;
            if self.energy > 9 {
                self.energy = 0;
                self.flashed = step;
                return true;
            }
        }
        false
    }
}

pub struct OctoModel {
    octos: Vec<Octo>,
}

impl OctoModel {
    pub fn new(data: &str) -> OctoModel {
        let mut reader = reader::BufReader::open(data.to_string()).unwrap();
        let mut buffer = String::new();
        let mut octos: Vec<Octo> = Vec::<Octo>::with_capacity(100);
        let mut y = 0;
        let mut x;
        while let Some(line) = reader.read_line(&mut buffer) {
            x = 0;
            for h in line
                .unwrap()
                .trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
            {
                let h: i8 = h.try_into().unwrap();
                octos.push(Octo::new((x, y), h));
                x += 1;
            }
            y += 1;
        }
        OctoModel { octos }
    }
    pub fn reset(&mut self) {
        for o in &mut self.octos {
            o.reset();
        }
    }
    pub fn step(&mut self, step: i32) -> i32 {
        let mut count = 0;
        let mut flashed: Vec<(i16, i16)> = Vec::<(i16, i16)>::with_capacity(100);
        for o in &mut self.octos {
            if o.step(step) {
                count += 1;
                flashed.push(o.coord());
            }
        }
        let mut c;
        let mut pos;
        while let Some(coord) = flashed.pop() {
            for x in -1..=1 {
                for y in -1..=1 {
                    if x == 0 && y == 0 {
                        continue;
                    }
                    c = ((coord.0 + x), (coord.1 + y));
                    if c.0 >= 0 && c.0 < 10 && c.1 >= 0 && c.1 < 10 {
                        pos = c.0 + c.1 * 10;
                        if self.octos[pos as usize].step(step) {
                            count += 1;
                            flashed.push(self.octos[pos as usize].coord());
                        }
                    }
                }
            }
        }
        count
    }
    pub fn multi_step(&mut self, steps: i32) -> i32 {
        let mut count = 0;
        for step in 1..=steps {
            count += self.step(step);
        }
        self.reset();
        count
    }
    pub fn get_sync(&mut self) -> i32 {
        let mut step = 1;
        loop {
            if self.step(step) == 100 {
                break;
            }
            step += 1;
        }
        self.reset();
        step
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let mut o = OctoModel::new("data/test/input11");
        assert_eq!(1656, o.multi_step(100));
    }

    #[test]
    fn part_two() {
        let mut o = OctoModel::new("data/test/input11");
        assert_eq!(195, o.get_sync());
    }
}
