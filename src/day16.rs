use std::collections::HashMap;

fn to_vec(input: &str) -> String {
    let hex_to_b: HashMap<String, String> = vec![
        ("0".to_string(), "0000".to_string()),
        ("1".to_string(), "0001".to_string()),
        ("2".to_string(), "0010".to_string()),
        ("3".to_string(), "0011".to_string()),
        ("4".to_string(), "0100".to_string()),
        ("5".to_string(), "0101".to_string()),
        ("6".to_string(), "0110".to_string()),
        ("7".to_string(), "0111".to_string()),
        ("8".to_string(), "1000".to_string()),
        ("9".to_string(), "1001".to_string()),
        ("A".to_string(), "1010".to_string()),
        ("B".to_string(), "1011".to_string()),
        ("C".to_string(), "1100".to_string()),
        ("D".to_string(), "1101".to_string()),
        ("E".to_string(), "1110".to_string()),
        ("F".to_string(), "1111".to_string()),
    ]
    .into_iter()
    .collect();
    input
        .chars()
        .into_iter()
        .map(|c| hex_to_b[&c.to_string()].clone())
        .collect()
}

enum Packet {
    Literal {
        version: usize,
        value: usize,
    },
    Operator {
        version: usize,
        type_id: usize,
        packets: Vec<Packet>,
    },
}

impl Packet {
    pub fn from_str(binary: &str, pos: usize) -> (Packet, usize) {
        let mut moving_pos = pos;
        let v = usize::from_str_radix(&binary[moving_pos..moving_pos + 3], 2).unwrap();
        moving_pos += 3;
        let t = usize::from_str_radix(&binary[moving_pos..moving_pos + 3], 2).unwrap();
        moving_pos += 3;
        // literal value!
        if t == 4 {
            let mut values = vec![];
            while &binary[moving_pos..moving_pos + 1] == "1" {
                moving_pos += 1;
                values.push(&binary[moving_pos..moving_pos + 4]);
                moving_pos += 4;
            }
            moving_pos += 1;
            values.push(&binary[moving_pos..moving_pos + 4]);
            moving_pos += 4;
            (
                Packet::Literal {
                    version: v,
                    value: usize::from_str_radix(&values.join(""), 2).unwrap(),
                },
                moving_pos,
            )
        } else {
            // parse operator
            let i = &binary[moving_pos..moving_pos + 1];
            moving_pos += 1;
            if i == "0" {
                let packet_length =
                    usize::from_str_radix(&binary[moving_pos..moving_pos + 15], 2).unwrap();
                moving_pos += 15;
                let mut seen_length = 0;
                let mut sub_packets = vec![];
                while seen_length < packet_length {
                    let (new_packet, new_pos) = Packet::from_str(binary, moving_pos);
                    sub_packets.push(new_packet);
                    seen_length += new_pos - moving_pos;
                    moving_pos = new_pos;
                }
                (
                    Packet::Operator {
                        version: v,
                        type_id: t,
                        packets: sub_packets,
                    },
                    moving_pos,
                )
            } else {
                let num_sub_packets =
                    usize::from_str_radix(&binary[moving_pos..moving_pos + 11], 2).unwrap();
                moving_pos += 11;
                let mut sub_packets = vec![];
                for _ in 0..num_sub_packets {
                    let (new_packet, new_pos) = Packet::from_str(binary, moving_pos);
                    sub_packets.push(new_packet);
                    moving_pos = new_pos;
                }
                (
                    Packet::Operator {
                        version: v,
                        type_id: t,
                        packets: sub_packets,
                    },
                    moving_pos,
                )
            }
        }
    }

    pub fn get_packet_versions(&self) -> Vec<&usize> {
        match self {
            Packet::Operator {
                version,
                type_id: _,
                packets,
            } => {
                let mut v: Vec<&usize> = packets
                    .iter()
                    .flat_map(|p| p.get_packet_versions())
                    .collect();
                v.push(version);
                v
            }
            Packet::Literal { version, value: _ } => vec![version],
        }
    }

    pub fn get_packet_value(&self) -> usize {
        match self {
            Packet::Operator {
                version: _,
                type_id,
                packets,
            } => {
                let packet_values: Vec<usize> =
                    packets.iter().map(|p| p.get_packet_value()).collect();
                match type_id {
                    0 => packet_values.iter().sum(),
                    1 => packet_values.iter().product(),
                    2 => *packet_values.iter().min().unwrap(),
                    3 => *packet_values.iter().max().unwrap(),
                    5 => {
                        if packet_values[0] > packet_values[1] {
                            1
                        } else {
                            0
                        }
                    }
                    6 => {
                        if packet_values[0] < packet_values[1] {
                            1
                        } else {
                            0
                        }
                    }
                    7 => {
                        if packet_values[0] == packet_values[1] {
                            1
                        } else {
                            0
                        }
                    }
                    _ => 0,
                }
            }
            Packet::Literal { version: _, value } => *value,
        }
    }
}

#[aoc(day16, part1)]
fn day16_1(input: &str) -> usize {
    let binary = to_vec(input);
    let (packet, _) = Packet::from_str(&binary, 0);
    let mut sum = 0;
    for v in packet.get_packet_versions() {
        sum += *v
    }
    sum
}

#[aoc(day16, part2)]
fn day16_2(input: &str) -> usize {
    let binary = to_vec(input);
    let (packet, _) = Packet::from_str(&binary, 0);
    packet.get_packet_value()
}
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "A0016C880162017C3686B18A3D4780";

    #[test]
    fn test_input() {
        assert_eq!(day16_1(&TEST_INPUT), 31);
        assert_eq!(day16_2(&TEST_INPUT), 0);
    }

    #[test]
    fn test_literal_packet() {
        let binary = to_vec("D2FE28");
        let (packet, _) = Packet::from_str(&binary, 0);
        if let Packet::Literal { version, value } = packet {
            assert_eq!(version, 6);
            assert_eq!(value, 2021);
        }
    }

    #[test]
    fn test_len_operator() {
        let binary = to_vec("38006F45291200");
        let (packet, _) = Packet::from_str(&binary, 0);
        if let Packet::Operator {
            version,
            type_id,
            packets,
        } = packet
        {
            assert_eq!(version, 1);
            assert_eq!(type_id, 6);
            assert_eq!(packets.len(), 2);
            if let Packet::Literal { version: _, value } = packets[0] {
                assert_eq!(value, 10)
            }
            if let Packet::Literal { version: _, value } = packets[1] {
                assert_eq!(value, 20)
            }
        }
    }

    #[test]
    fn test_num_operator() {
        let binary = to_vec("EE00D40C823060");
        let (packet, _) = Packet::from_str(&binary, 0);
        if let Packet::Operator {
            version,
            type_id,
            packets,
        } = packet
        {
            assert_eq!(version, 7);
            assert_eq!(type_id, 3);
            assert_eq!(packets.len(), 3);
            if let Packet::Literal { version: _, value } = packets[0] {
                assert_eq!(value, 1)
            }
            if let Packet::Literal { version: _, value } = packets[1] {
                assert_eq!(value, 2)
            }
            if let Packet::Literal { version: _, value } = packets[2] {
                assert_eq!(value, 3)
            }
        }
    }
}
