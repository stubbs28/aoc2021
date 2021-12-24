use std::fs::File;
use std::io::{BufRead, BufReader};
#[path = "utils/reader.rs"]
mod reader;

pub struct PacketData {
    data: Vec<u8>,
}

impl PacketData {
    pub fn new(data: Vec<u8>) -> PacketData {
        PacketData { data }
    }
    pub fn unpack(&self, index: &mut usize, mask_len: &mut u32, field_len: u32) -> u32 {
        let mut staging: u32 = self.data[*index] as u32;
        while *mask_len < field_len {
            staging = staging << 8;
            if *index < self.data.len() - 1 {
                *index += 1;
                staging = staging | self.data[*index] as u32;
            }
            *mask_len += 8;
        }
        *mask_len -= field_len;
        let shifted = staging >> *mask_len;
        let mask: u32 = !(0xFFFFFFFF << field_len);
        shifted & mask
    }
    pub fn unpack_raw(&self, index: &mut usize, start_bit: &mut u32) -> (u64, usize) {
        let mut size = 0;
        let mut raw: u64 = 0;
        let mut staging;
        loop {
            staging = self.unpack(index, start_bit, 5);
            size += 5;
            raw = (raw << 4) | (staging & 0xF) as u64;
            if ((staging >> 4) & 1) == 0 {
                break;
            }
        }
        (raw, size)
    }
}

#[derive(Debug)]
pub struct Packet {
    version: u32,
    ptype: u32,
    len: usize,
    operator_data: Vec<Packet>,
    literal_data: u64,
}

impl Packet {
    pub fn new(index: &mut usize, start_bit: &mut u32, data: &PacketData) -> Packet {
        let version = data.unpack(index, start_bit, 3);
        let ptype = data.unpack(index, start_bit, 3);
        let mut len = 6;
        let mut literal_data = 0;
        let mut operator_data = Vec::<Packet>::new();
        if ptype == 4 {
            let ld = data.unpack_raw(index, start_bit);
            literal_data = ld.0;
            len += ld.1;
        } else {
            len += 1;
            if data.unpack(index, start_bit, 1) == 0 {
                let mut bcount = data.unpack(index, start_bit, 15) as usize;
                len += 15;
                while bcount > 0 {
                    let p = Packet::new(index, start_bit, data);
                    bcount -= p.len();
                    len += p.len();
                    operator_data.push(p);
                }
            } else {
                let pcount = data.unpack(index, start_bit, 11) as usize;
                len += 11;
                while operator_data.len() < pcount {
                    let p = Packet::new(index, start_bit, data);
                    len += p.len();
                    operator_data.push(p);
                }
            }
        }
        let p = Packet {
            version,
            ptype,
            len,
            operator_data,
            literal_data,
        };
        p
    }
    pub fn version_sum(&self) -> u32 {
        let mut sum = self.version;
        for p in &self.operator_data {
            sum += p.version_sum();
        }
        sum
    }
    pub fn value(&self) -> u64 {
        let mut value = self.literal_data;
        for (pos, p) in self.operator_data.iter().enumerate() {
            match self.ptype {
                0 => value += p.value(),
                1 => {
                    if pos == 0 {
                        value = p.value();
                    } else {
                        value *= p.value();
                    }
                }
                2 => {
                    let v = p.value();
                    if pos == 0 || value > v {
                        value = v;
                    }
                }
                3 => {
                    let v = p.value();
                    if pos == 0 || value < v {
                        value = v;
                    }
                }
                4 => value = self.literal_data,
                5 => {
                    if pos == 0 {
                        value = p.value();
                        continue;
                    }
                    if value > p.value() {
                        value = 1;
                    } else {
                        value = 0;
                    }
                }
                6 => {
                    if pos == 0 {
                        value = p.value();
                        continue;
                    }
                    if value < p.value() {
                        value = 1;
                    } else {
                        value = 0;
                    }
                }
                7 => {
                    if pos == 0 {
                        value = p.value();
                        continue;
                    }
                    if value == p.value() {
                        value = 1;
                    } else {
                        value = 0;
                    }
                }
                _ => panic!("unexpected packet type"),
            }
        }
        value
    }
    pub fn len(&self) -> usize {
        self.len
    }
}

pub struct BITS {
    packet: Packet,
}

impl BITS {
    pub fn new(data: &str) -> BITS {
        let mut index: usize = 0;
        let file = File::open(data).expect("unable to open the file");
        let f = BufReader::new(file);
        let mut data: Vec<u8> = Vec::<u8>::new();
        let mut byte: String = String::new();
        f.lines()
            .flat_map(|l| l.expect("lines failed").chars().collect::<Vec<_>>())
            .for_each(|c| {
                if index % 2 == 0 {
                    byte = c.to_string();
                } else {
                    byte.push(c);
                    data.push(u8::from_str_radix(&byte, 16).unwrap());
                }
                index += 1;
            });
        index = 0;
        let mut last_bit = 8;
        let packet_data = PacketData::new(data);
        let p = Packet::new(&mut index, &mut last_bit, &packet_data);
        BITS { packet: p }
    }
    pub fn version_sum(&self) -> u32 {
        self.packet.version_sum()
    }
    pub fn value(&self) -> u64 {
        self.packet.value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_packet_literal() {
        let p = BITS::new("data/test/input16.1");
        assert_eq!(6, p.packet.version);
        assert_eq!(4, p.packet.ptype);
        assert_eq!(2021, p.packet.literal_data);
    }

    #[test]
    fn single_packet_operator_1() {
        let p = BITS::new("data/test/input16.2");
        assert_eq!(1, p.packet.version);
        assert_eq!(6, p.packet.ptype);
        let subp = p.packet.operator_data;
        assert_eq!(2, subp.len());
        assert_eq!(10, subp[0].literal_data);
        assert_eq!(20, subp[1].literal_data);
    }

    #[test]
    fn single_packet_operator_2() {
        let p = BITS::new("data/test/input16.3");
        assert_eq!(7, p.packet.version);
        assert_eq!(3, p.packet.ptype);
        let subp = p.packet.operator_data;
        assert_eq!(3, subp.len());
        assert_eq!(1, subp[0].literal_data);
        assert_eq!(2, subp[1].literal_data);
        assert_eq!(3, subp[2].literal_data);
    }

    #[test]
    fn single_packet_operator_3() {
        /* A004A801A8002F478 represents an operator packet (version 4) which
         * contains an operator packet (version 1) which
         * contains an operator packet (version 5) which
         * contains a literal value (version 6);
         */
        let b = BITS::new("data/test/input16.4");
        let mut p = &b.packet;
        assert_eq!(4, p.version);
        p = &p.operator_data[0];
        assert_eq!(1, p.version);
        p = &p.operator_data[0];
        assert_eq!(5, p.version);
        p = &p.operator_data[0];
        assert_eq!(6, p.version);
        assert_eq!(16, b.packet.version_sum());
    }
    #[test]
    fn single_packet_operator_4() {
        /* 620080001611562C8802118E34 represents an operator packet (version 3) which
         * contains two sub-packets;
         * each sub-packet is an operator packet that
         * contains two literal values.
         * This packet has a version sum of 12.
         */
        let b = BITS::new("data/test/input16.5");
        let p = &b.packet;
        assert_eq!(3, p.version);
        assert_eq!(4, p.operator_data[0].operator_data[0].ptype);
        assert_eq!(4, p.operator_data[1].operator_data[0].ptype);
        assert_eq!(12, b.packet.version_sum());
    }

    #[test]
    fn single_packet_operator_5() {
        /* C0015000016115A2E0802F182340 represents an operator packet (version 3) which
         * contains two sub-packets;
         * each sub-packet is an operator packet that
         * contains two literal values.
         * This packet has a version sum of 12.
         */
        let b = BITS::new("data/test/input16.6");
        let p = &b.packet;
        assert_eq!(4, p.operator_data[0].operator_data[0].ptype);
        assert_eq!(4, p.operator_data[1].operator_data[0].ptype);
        assert_eq!(23, b.packet.version_sum());
    }

    #[test]
    fn single_packet_operator_6() {
        let p = BITS::new("data/test/input16.7");
        assert_eq!(31, p.packet.version_sum());
    }
}
