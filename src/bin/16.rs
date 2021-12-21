
use advent_of_code::parse_data_file;
use anyhow::anyhow;

fn parse_input() -> anyhow::Result<Vec<u8>> {
    let input_string = parse_data_file("test.txt")?;
    // let input_string = parse_data_file("16.txt")?;



    Ok((0..input_string.trim().len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&input_string[i..=i+1], 16).expect("Expected input to include hex string!"))
        .collect())
             
}

fn main() -> anyhow::Result<()> {
    let input = parse_input()?;

    let mut input_iter = input.into_iter();
    let mut remaining = 8;
    let mut current = input_iter.next();
    let mut answer = 0;
    loop {
        if current.is_none() {
            break;
        }
        let mut current_val = current.unwrap();

        let version_pull = pull_bits(3, remaining, current_val, &mut input_iter)?;
        answer += version_pull.value as u32;
        let id_pull = pull_bits(3, version_pull.new_remaining, version_pull.current.expect("Expect ID"), &mut input_iter)?;
        let id = id_pull.value;
        current_val = id_pull.current.unwrap();
        remaining = id_pull.new_remaining;
        println!("Version: {}, ID: {}", version_pull.value, &id);
        if version_pull.value == 0 && id == 0 {
            break;
        }
        match id {
            // literal number
            4 => {
                let (val, pull, _) = pull_literal_number(remaining, current_val, &mut input_iter)?;
                current = pull.current;
                remaining = pull.new_remaining;
                // TODO: DO something with val
            }
            // Operator
            _ => {
                println!("TEST");
                let type_pull = pull_bits(1, remaining, current_val, &mut input_iter)?;
                let length_bits = if type_pull.value == 1 { 11 } else if type_pull.value == 0 { 15 } else { panic!("WFT!") };
                current_val = type_pull.current.expect("Expect there to be a current value after operator type pull!");
                let mut pull = pull_bits(length_bits, type_pull.new_remaining, current_val, &mut input_iter)?;
                let (mut packets_remaining, mut literal_bits) = if type_pull.value == 1 {
                    (pull.value, 0)
                } else {
                    (0, pull.value)
                };
                while packets_remaining > 0 || literal_bits > 0 {
                    println!("Packets: {}, bits: {}", packets_remaining, literal_bits);
                    println!("Literal Pull: {:?}", pull);
                    current_val = pull.current.expect("Expect there to be a current value after operator length pull!");

                    // TODO I should really re-use this from above
                    let version_pull = pull_bits(3, pull.new_remaining, current_val, &mut input_iter)?;
                    answer += version_pull.value as u32;
                    let id_pull = pull_bits(3, version_pull.new_remaining, version_pull.current.expect("Expect ID"), &mut input_iter)?;
                    if id_pull.value != 4 { println!("Expecting literal packet headers!"); }
                    current_val = id_pull.current.expect("Expect there to be a current value after operator length pull!");

                    let (_val, literals_pull, bits_pulled) = pull_literal_number(id_pull.new_remaining, current_val, &mut input_iter)?;
                    pull = literals_pull;
                    if literal_bits > 0 { literal_bits -= bits_pulled }
                    if packets_remaining > 0 { packets_remaining -= 1 }
                }

                current = pull.current;
                remaining = pull.new_remaining;
            }
        }
    }

    println!("The answer is {}", answer);
    Ok(())
}

fn pull_literal_number<T>(remaining: u32, current: u8, iter: &mut T) -> anyhow::Result<(u32, Pull, u32)> 
where
    T: Iterator<Item = u8>
{
    let mut val = 0;
    let mut bits_pulled = 0;
    let mut last_pull = None;
    let mut inner_current = current;
    let mut inner_remaining = remaining;
    loop {
        bits_pulled += 5;
        // std::thread::sleep(std::time::Duration::from_millis(1000));
        let header_pull = pull_bits(1, inner_remaining, inner_current, iter)?;
        let val_pull = pull_bits(4, header_pull.new_remaining, header_pull.current.unwrap(), iter)?;
        val += val_pull.value;
        inner_current = val_pull.current.unwrap();
        inner_remaining = val_pull.new_remaining;
        if header_pull.value == 0 { 
            last_pull = Some(val_pull);
            break; 
        }
    }
    println!("Returning literal: {}", val);
    last_pull.ok_or(anyhow!("Failed to assign last pull!")).map(|pull| (val, pull, bits_pulled))
}

#[derive(Debug)]
struct Pull {
    current: Option<u8>,
    new_remaining: u32,
    value: u32
}

fn pull_bits<T>(mut bits_to_pull: u32, remaining: u32, mut current: u8, iter: &mut T) -> anyhow::Result<Pull> 
where
    T: Iterator<Item = u8>
{
    std::thread::sleep(std::time::Duration::from_millis(1000));
    println!("Current: {:b}, remaining: {}, bits to pull: {}", current, remaining, bits_to_pull);
    let mut output = 0;
    if remaining > 0 {
        if bits_to_pull <= remaining {
            let (new_current, val) = rotate_left(current, bits_to_pull);
            output += val as u32;
            return Ok(Pull {
                current: Some(new_current),
                value: output,
                new_remaining: remaining - bits_to_pull
            });
        } else {
            let (_, val) = rotate_left(current, remaining);
            output += val as u32;
            current = iter.next().ok_or(anyhow!("Failed to rollover remaining!"))?;
            println!("Temp: {:b}", current);
            bits_to_pull -= remaining;
        }
    }

    while bits_to_pull >= 8 {
        output <<= 8;
        output += current as u32; 
        current = iter.next().ok_or(anyhow!("Failed to rollover current!"))?;
        println!("Temp2: {:b}", current);
        bits_to_pull -= 8;
    }

    output <<= bits_to_pull;
    let (new_current, val) = rotate_left(current, bits_to_pull);
    output += val as u32;

    Ok(Pull {
        current: Some(new_current), 
        new_remaining: 8 - bits_to_pull,
        value: output
    })
}

fn rotate_left(mut current: u8, bits: u32) -> (u8, u8) {
    let mut mask = 0;
        0;
    for _ in 0..bits {
        mask <<= 1;
        mask += 1;
    }
    mask <<= 8 - bits;
    
    let mut val = current & mask;
    val >>= 8 - bits;
    current <<= bits;

    (current, val)
}
