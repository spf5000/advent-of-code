use std::collections::HashSet;

use crate::parse_data_file;
use anyhow::bail;

pub struct Day6 { }

impl Default for Day6 {
    fn default() -> Self {
        Self { }
    }
}

impl crate::DayAnswers for Day6 {
    fn get_answer(&self, question: crate::model::Question) -> anyhow::Result<()> {
        crate::run_question_answers(part1, part2, question)
    }
}

fn part1() -> anyhow::Result<()> {
    let input = parse_data_file(super::YEAR, 6)?;
    println!("Answer is {}", part1_helper(&input)?);
    Ok(())
}

fn part2() -> anyhow::Result<()> {
    let input = parse_data_file(super::YEAR, 6)?;
    println!("Answer is {}", part2_helper(&input)?);
    Ok(())
}

fn part1_helper(input: &str) -> anyhow::Result<usize> {
    helper(input, 4)
}

fn part2_helper(input: &str) -> anyhow::Result<usize> {
    helper(input, 14)
}

fn helper(input: &str, distinct_chars: usize) -> anyhow::Result<usize> {
    for i in 0..input.len() {
        let chars = &input[i..i+distinct_chars].chars().collect::<HashSet<char>>();
        if chars.len() == distinct_chars {
            return Ok(i+distinct_chars)
        }
    }

    bail!("Didn't find the start-of-packet marker!")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let test_input = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        for (input, expected_output) in test_input {
            let output = part1_helper(input);
            assert!(output.is_ok());
            assert_eq!(expected_output, output.unwrap())
        }
    }

    #[test]
    fn part2_test() {
        let test_input = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];

        for (input, expected_output) in test_input {
            let output = part2_helper(input);
            assert!(output.is_ok());
            assert_eq!(expected_output, output.unwrap())
        }
    }
}
