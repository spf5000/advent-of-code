use advent_of_code::parse_data_file;
use anyhow::anyhow;
use std::iter::Extend;

const TYPE_MASK: u32 = 7; // 7 = 00000111
const LITERAL_GROUP_PREFIX_MASK: u32 = 16; // 7 = 10000
const LITERAL_GROUP_SUFFIX_MASK: u32 = 15; // 7 = 1111
const HEADER_LEN: u8 = 6;

fn parse_input() -> anyhow::Result<Vec<u8>> {
    // let input_string = parse_data_file("test.txt")?;
    let input_string = parse_data_file("16.txt")?;

    Ok((0..input_string.trim().len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&input_string[i..=i + 1], 16)
                .expect("Expected input to include hex string!")
        })
        .collect())
}

fn main() -> anyhow::Result<()> {
    let input = parse_input()?;

    let mut remaining = 8;
    let mut answer = 0;
    let mut i = 0;

    let (packet, _) = parse_packet(&mut i, &mut remaining, &input)?;

    println!("The answer to puzzle 1 is {}", count_versions(&packet));
    println!("The answer to puzzle 2 is {}", perform_operations(&packet));
    Ok(())
}

fn count_versions(packet: &Packet) -> u32 {
    match &packet.value {
        PacketValue::Literal(_) => packet.version as u32,
        PacketValue::Operator(subpackets) => subpackets
            .iter()
            .fold(packet.version as u32, |acc, subpacket| {
                count_versions(subpacket) + acc
            }),
    }
}

fn perform_operations(packet: &Packet) -> u128 {
    match &packet.value {
        PacketValue::Literal(value) => *value as u128,
        PacketValue::Operator(subpackets) => match packet.type_id {
            // Sum
            0 => subpackets
                .iter()
                .fold(0, |acc, subpacket| acc + perform_operations(subpacket)),
            // Product
            1 => subpackets
                .iter()
                .fold(1, |acc, subpacket| acc * perform_operations(subpacket)),
            // Min
            2 => subpackets.iter().fold(u128::MAX, |acc, subpacket| {
                let subpacket_val = perform_operations(subpacket);
                if acc < subpacket_val {
                    acc
                } else {
                    subpacket_val
                }
            }),
            // Max
            3 => subpackets.iter().fold(u128::MIN, |acc, subpacket| {
                let subpacket_val = perform_operations(subpacket);
                if acc > subpacket_val {
                    acc
                } else {
                    subpacket_val
                }
            }),
            // Greater than
            5 => {
                let first = perform_operations(&subpackets[0]);
                let last = perform_operations(&subpackets[1]);
                if first > last {
                    1
                } else {
                    0
                }
            },
            // Less than
            6 => {
                let first = perform_operations(&subpackets[0]);
                let last = perform_operations(&subpackets[1]);
                if first < last {
                    1
                } else {
                    0
                }
            },
            // Equals
            7 => {
                let first = perform_operations(&subpackets[0]);
                let last = perform_operations(&subpackets[1]);
                if first == last {
                    1
                } else {
                    0
                }
            }
            _ => panic!("Unexpected packet type id!: {}", packet.type_id),
        },
    }
}

#[derive(Debug)]
enum PacketValue {
    Literal(u32),
    Operator(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    value: PacketValue,
}

fn parse_packet(
    i: &mut usize,
    remaining: &mut u8,
    bytes: &Vec<u8>,
) -> anyhow::Result<(Packet, u32)> {
    let (version, type_id) = read_header(i, remaining, bytes)?;
    match type_id {
        4 => {
            let (value, bits) = pull_literal_number(i, remaining, bytes)?;
            let packet = Packet {
                version,
                type_id,
                value: PacketValue::Literal(value),
            };
            Ok((packet, (bits + HEADER_LEN) as u32))
        }
        // Operators
        _ => {
            let mut bits_used = HEADER_LEN as u32;
            let length_type_id = pull_bits(1, i, remaining, bytes)?;
            let (mut packets_remaining, mut bits_remaining) = if length_type_id == 0 {
                bits_used += 16;
                (0, pull_bits(15, i, remaining, bytes)?)
            } else if length_type_id == 1 {
                bits_used += 12;
                (pull_bits(11, i, remaining, bytes)?, 0)
            } else {
                return Err(anyhow!("Operator bit should be 0 or 1! {}", length_type_id));
            };

            let mut value = Vec::new();
            while packets_remaining > 0 || bits_remaining > 0 {
                let (subpacket, bits) = parse_packet(i, remaining, bytes)?;
                value.push(subpacket);
                bits_used += bits as u32;
                if packets_remaining > 0 {
                    packets_remaining -= 1
                }
                if bits_remaining > 0 {
                    bits_remaining -= bits as u32
                }
            }
            let packet = Packet {
                version,
                type_id,
                value: PacketValue::Operator(value),
            };
            Ok((packet, bits_used))
        }
    }
}

fn read_header(i: &mut usize, remaining: &mut u8, bytes: &Vec<u8>) -> anyhow::Result<(u8, u8)> {
    let header = pull_bits(6, i, remaining, bytes)?;
    let type_id = (header & TYPE_MASK) as u8;
    let version = (header >> 3) as u8;
    Ok((version, type_id))
}

fn pull_literal_number(
    i: &mut usize,
    remaining: &mut u8,
    bytes: &Vec<u8>,
) -> anyhow::Result<(u32, u8)> {
    let mut output_val = 0;
    let mut used_bits = 0;
    loop {
        output_val <<= 4;
        used_bits += 5;
        let chunk = pull_bits(5, i, remaining, bytes)?;
        let value = chunk & LITERAL_GROUP_SUFFIX_MASK;
        output_val += value;
        if chunk & LITERAL_GROUP_PREFIX_MASK == 0 {
            break;
        }
    }

    Ok((output_val, used_bits))
}

fn pull_bits(
    mut bits_to_pull: u8,
    i: &mut usize,
    remaining: &mut u8,
    bytes: &Vec<u8>,
) -> anyhow::Result<u32> {
    let mut output = 0;
    if bits_to_pull > *remaining {
        // pull the remaining bits for the current index in the bytes
        let remaining_mask = create_mask(8 - *remaining, 8);
        output += (bytes[*i] & remaining_mask) as u32;
        bits_to_pull -= *remaining;
        *remaining = 8;
        *i += 1;

        // pull each byte from the array until we're on the last byte with remaining bits
        while bits_to_pull > 8 {
            output <<= 8;
            output += bytes[*i] as u32;
            bits_to_pull -= 8;
            *i += 1;
        }
    }

    // Handle the remaining bits
    output <<= bits_to_pull;
    let mask_start = 8 - *remaining;
    let mask = create_mask(mask_start, mask_start + bits_to_pull);
    *remaining -= bits_to_pull;

    let mut value = bytes[*i] & mask;
    value >>= *remaining;

    output += value as u32;
    Ok(output)
}

fn create_mask(start: u8, end: u8) -> u8 {
    let mut output = 0;
    for i in 0..8 {
        output <<= 1;
        if i >= start && i < end {
            output += 1;
        }
    }
    output
}
