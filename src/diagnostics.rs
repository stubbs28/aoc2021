#[path="utils/reader.rs"] mod reader;

pub struct Power {
    readout: String,
}

impl Power {
    pub fn new(readout: String) -> Power {
        Power {
            readout: readout,
        }
    }

    pub fn consumption(&self) -> u32 {
        let mut reader = reader::BufReader::open(&self.readout).unwrap();
        let mut buffer = String::new();
        let mut common = Vec::<u32>::new();
        let mut count = 0;
        let mut mask: u32;
        common.resize(12, 0);
        while let Some(line) = reader.read_line(&mut buffer) {
            count += 1;
            let data = u32::from_str_radix(line.unwrap().trim(), 2).unwrap();
            mask = 1;
            for i in &mut common {
                if data & mask > 0 {
                    *i += 1;
                }
                mask = mask << 1;
            }
        }
        let mut gamma = 0;
        mask = 1;
        for i in common {
            if i > (count / 2) {
                gamma |= mask;
            }
            mask = mask << 1;
        }
        gamma * (!gamma&0xFFF)
    }
}
