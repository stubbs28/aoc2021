#[path="utils/reader.rs"] mod reader;

pub struct Power {
    readout: String,
}

pub fn frequent(ones: i32, zeros: i32) -> u32 {
    if ones >= zeros {
        return 1 ;
    } 
    return 0;
}

pub fn infrequent(ones: i32, zeros: i32) -> u32 {
    if ones + zeros == 1 {
        return frequent(ones, zeros);
    }
    if ones >= zeros {
        return 0 ;
    } 
    return 1;
}

pub type Freq = fn(i32, i32) -> u32;

impl Power {
    pub fn new(readout: String) -> Power {
        Power {
            readout: readout,
        }
    }

    pub fn life_support(&self) -> u32 {
        let oxygen = self.filter(12, 0, frequent);
        let co2 = self.filter(12, 0, infrequent);
        oxygen * co2
    }

    pub fn filter(&self, pos: u32, val: u32, f: Freq) -> u32 {
        if pos == 0 {
            return val;
        }
        let mut reader = reader::BufReader::open(&self.readout).unwrap();
        let mut buffer = String::new();
        let mut ones = 0;
        let mut zeros = 0;
        while let Some(line) = reader.read_line(&mut buffer) {
            let data = u32::from_str_radix(line.unwrap().trim(), 2).unwrap();
            if (data >> pos) == val {
                if ((data >> (pos - 1)) & 1) > 0  {
                    ones += 1;
                } else {
                    zeros += 1;
                }
            }
        }
        let mut v = val << 1;
        v += f(ones, zeros);
        self.filter(pos - 1, v, f)
    }

    pub fn consumption(&self) -> u32 {
        let mut reader = reader::BufReader::open(&self.readout).unwrap();
        let mut buffer = String::new();
        let mut freq = Vec::<u32>::new();
        let mut count = 0;
        let mut mask: u32;
        freq.resize(12, 0);
        while let Some(line) = reader.read_line(&mut buffer) {
            count += 1;
            let data = u32::from_str_radix(line.unwrap().trim(), 2).unwrap();
            mask = 1;
            for i in &mut freq {
                if data & mask > 0 {
                    *i += 1;
                }
                mask = mask << 1;
            }
        }
        let mut gamma = 0;
        mask = 1;
        for i in freq {
            if i > (count / 2) {
                gamma |= mask;
            }
            mask = mask << 1;
        }
        gamma * (!gamma&0xFFF)
    }
}
