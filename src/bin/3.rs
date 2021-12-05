use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    data_path.push("data/3.txt");

    let data = std::fs::read_to_string(&data_path)?;

    // Use peekable iteractor to get the first line 
    // and initialize the vector
    let mut line_iter = data.lines().peekable();
    let first_line = line_iter.peek().ok_or(anyhow::Error::msg("First line of data is empty!"))?;
    let mut bit_count: Vec<u64> = Vec::new();
    bit_count.resize(first_line.len(), 0);

    // Get the count of 1's (vs. 0's) per bit element per line.
    let mut line_count = 0;
    for line in line_iter {
        line_count += 1;
        for (c, i) in line.chars().zip(0..) {
            match c {
                '0' => (), // No-Op
                '1' => bit_count[i] += 1,
                _ => panic!("Non-binary character in input: {}", c)
            };
        }
    }

    // if the count > (line_count / 1), then the majority of the values
    // are 1's, else 0's
    let mut gamma: u64 = 0;
    let mut epsilon: u64 = 0;
    for count in bit_count {
        gamma <<= 1;
        epsilon <<= 1;
        println!("Count: {}, line_count: {}", count, line_count);
        if count >= line_count / 2 {
            gamma += 1;
        } else {
            epsilon += 1;
        }
        println!("Gamma:   {:#016b}", gamma);
        println!("Epsilon: {:#016b}", epsilon);
    }


    println!("Answer: {}", gamma * epsilon);

    Ok(())
}
