pub struct Day3 {}

impl Default for Day3 {
    fn default() -> Self {
        Self { }
    }
}

impl crate::DayAnswers for Day3 {
    fn get_answer(&self, question: crate::model::Question) -> anyhow::Result<()> {
        crate::run_question_answers(part1, part2, question)
    }
}

use crate::parse_data_file;

fn part1() -> anyhow::Result<()> {
    let data = parse_data_file(super::YEAR, 3)?;

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

fn part2() -> anyhow::Result<()> {
    let data = parse_data_file(super::YEAR, 3)?;
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

