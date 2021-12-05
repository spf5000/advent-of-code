use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    data_path.push("data/3.txt");

    let data = std::fs::read_to_string(&data_path)?;
    let oxygen = get_rating(data.lines().collect(), true);
    let co2 = get_rating(data.lines().collect(), false);

    println!("Answer: {}", oxygen * co2);

    Ok(())
}

/// Takes a input lines 
fn get_rating(mut lines: Vec<&str>, greater_count_wins: bool) -> u64 {
    let mut char_index = 0;
    loop {
        let mut zeros = Vec::new();
        let mut ones = Vec::new();
        for line in lines {
            match &line[char_index..char_index+1] {
                "0" => zeros.push(line),
                "1" => ones.push(line),
                _ => panic!("None binary value in the input data! {}", line)
            }

        }

        match (greater_count_wins, zeros.len() > ones.len()) {
            (true, true) | (false, false) => lines = zeros,
            (true, false) | (false, true) => lines = ones,
        }

        char_index += 1;
        if lines.len() <= 1 { break; }
    }

    // Convert string to a binary number
    let output_str = lines.pop().unwrap();
    let mut output = 0;
    for binary_char in output_str.chars() {
        output <<= 1;
        match binary_char {
            '1' => output += 1,
            '0' => (), // No-Op
            _ => panic!("How does the output have non-binary data! {}", output_str)
        };
    }
    output
}
