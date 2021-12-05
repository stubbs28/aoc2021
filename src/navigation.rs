#[path="utils/reader.rs"] mod reader;
pub struct Helm {
    course: String,
    x: i32,
    y: i32,
}

impl Helm {
    pub fn new(course: String) -> Helm {
        Helm {
            course: course,
            x: 0,
            y: 0,

        }
    }

    pub fn location(&self) -> i32 {
        self.x * self.y
    }

    pub fn navigate(&mut self) {
        let mut reader = reader::BufReader::open(&self.course).unwrap();
        let mut buffer = String::new();
        while let Some(line) = reader.read_line(&mut buffer) {
            let step: Vec<&str> = line.unwrap().trim()
                .split(" ")
                .filter(|s| !s.is_empty())
                .collect();
            let delta: i32 = step[1].parse::<i32>().unwrap();
            match step[0] {
                "forward"=>self.x += delta,
                "down"=>self.y += delta,
                "up"=>self.y -= delta,
                _=>panic!("thats not a valid direction")
            }
        }
    }
}
