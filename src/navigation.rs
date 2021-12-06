#[path="utils/reader.rs"] mod reader;

pub struct Location {
    x: i64,
    y: i64,
    aim: i64,
}

impl Location {
    pub fn new_loc() -> Location {
        Location {
            x: 0,
            y: 0,
            aim: 0,
        }
    }

    pub fn hash(&self) -> i64 {
        self.x * self.y
    }

    pub fn translate(&mut self, dir: &str, delta: i64) {
        match dir {
            "forward"=>self.x += delta,
            "down"=>self.y += delta,
            "up"=>self.y -= delta,
            _=>panic!("thats not a valid direction")
        }
    }

    pub fn adv_translate(&mut self, dir: &str, delta: i64) {
        match dir {
            "forward"=>{
                self.x += delta;
                self.y += self.aim * delta;
            },
            "down"=>self.aim += delta,
            "up"=>self.aim -= delta,
            _=>panic!("thats not a valid direction")
        }
    }
}

pub struct Helm {
    course: String,
}

impl Helm {
    pub fn new(course: String) -> Helm {
        Helm {
            course: course,
        }
    }

    pub fn navigate(&mut self, aim: bool) -> i64{
        let mut reader = reader::BufReader::open(&self.course).unwrap();
        let mut buffer = String::new();
        let mut loc = Location::new_loc();
        while let Some(line) = reader.read_line(&mut buffer) {
            let step: Vec<&str> = line.unwrap().trim()
                .split(" ")
                .filter(|s| !s.is_empty())
                .collect();
            let delta: i64 = step[1].parse::<i64>().unwrap();
            if aim {
                loc.adv_translate(step[0], delta);
            } else {
                loc.translate(step[0], delta);
            }
        }
        loc.hash()
    }
}
