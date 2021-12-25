use std::fs;
#[path = "utils/point.rs"]
mod point;

pub struct Probe {
    p1: point::Point,
    p2: point::Point,
}

impl Probe {
    pub fn new(data: &str) -> Probe {
        let input: String = fs::read_to_string(data)
            .expect("unable to open file")
            .parse()
            .expect("unable to parse file");
        Probe::new_probe(input)
    }
    fn new_probe(mut input: String) -> Probe {
        let s_mut_str = input.as_mut_str();
        let coords: Vec<(i32, i32)> = s_mut_str
            .trim()
            .strip_prefix("target area: ")
            .unwrap()
            .split(", ")
            .map(|x| {
                let p: Vec<&str> = x.split_at(2).1.split("..").collect();
                (
                    p[0].parse::<i32>().expect("nan"),
                    p[1].parse::<i32>().expect("nan"),
                )
            })
            .collect();
        input.split(", ");
        Probe {
            p1: point::Point::new(coords[0].0, coords[1].0),
            p2: point::Point::new(coords[0].1, coords[1].1),
        }
    }
    pub fn max_peak(&self) -> i32 {
        let mut vel = self.target_y();
        let mut pos = 0;
        let mut steps = 0;
        while vel != 0 {
            pos += vel;
            steps += 1;
            vel -= 1;
        }
        pos
    }
    pub fn target_x(&self) -> Vec<(i32, i32)> {
        let mut drag = 1;
        if self.p1.x() < 0 {
            drag = -1;
        }
        let mut vel = 0;
        let mut pos = 0;
        let mut steps = 0;
        let mut velocities = Vec::<(i32, i32)>::new();
        // get the min vel/steps
        while !(pos >= self.p1.x() && pos <= self.p2.x()) {
            vel += drag;
            pos += vel;
            steps += 1;
        }
        // get the max vel/steps
        velocities.push((vel, steps));
        while pos >= self.p1.x() && pos <= self.p2.x() {
            vel += drag;
            steps += 1;
            velocities.push((vel, steps));
            pos += vel;
        }
        velocities
    }
    pub fn target_y(&self) -> i32 {
        let mut pos_v = 0;
        if self.p2.y() > 0 {
            // if the target is above us our init velocity should bring us right
            // to the top edge.
            pos_v = self.p2.y();
        }
        let mut neg_v = 0;
        if self.p1.y() < 0 {
            // vel - gravity because we want the vel from the prev step,
            // since this will be our final vel
            neg_v = (self.p1.y() + 1) * -1;
        }
        if pos_v > neg_v {
            return pos_v;
        }
        neg_v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let input = "target area: x=20..30, y=-10..-5".to_string();
        let p = Probe::new_probe(input);
        assert_eq!(point::Point::new(20, -10), p.p1);
        assert_eq!(point::Point::new(30, -5), p.p2);
        let x = p.target_x();
        assert_eq!(9, p.target_y());
        assert_eq!(45, p.max_peak());
    }
}
