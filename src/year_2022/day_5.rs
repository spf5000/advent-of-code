use std::str::FromStr;

use anyhow::bail;

use crate::parse_data_file;

pub struct Day5 { }

impl Default for Day5 {
    fn default() -> Self {
        Self { }
    }
}

impl crate::DayAnswers for Day5 {
    fn get_answer(&self, question: crate::model::Question) -> anyhow::Result<()> {
        crate::run_question_answers(part1, part2, question)
    }
}

#[derive(Debug, PartialEq)]
struct Input {
    starting_stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>
}

#[derive(Debug, PartialEq)]
struct Instruction {
    crates_to_move: u8,
    from: usize,
    to: usize
}

impl Instruction {
    fn perform_move(&self, stacks: &mut Vec<Vec<char>>, in_order: bool) -> anyhow::Result<()>{
        let from = &mut stacks[self.from - 1];
        let mut pulled_crates = Vec::new();
        for i in 0..self.crates_to_move {
            let c = from.pop().ok_or(anyhow::anyhow!("Failed to pop the {} crate off of {}!", i, self.from))?;
            pulled_crates.push(c);
        }
        if in_order {
            while let Some(c) = pulled_crates.pop() {
                stacks[self.to - 1].push(c);
            }
        } else {
            stacks[self.to - 1].append(&mut pulled_crates);
        }
        Ok(())
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // example: move <crates_to_move> from <starting_stack> to <ending_stack>
        let split = s.trim().split(' ').collect::<Vec<&str>>();
        if split.len() != 6 {
            bail!("Instruction string not valid! {}", s)
        } else {
            Ok(Self {
                crates_to_move: u8::from_str(split[1])?,
                from: usize::from_str(split[3])?,
                to: usize::from_str(split[5])?,
            })
        }
    }
}

fn part1() -> anyhow::Result<()> {
    let input = parse_input(parse_data_file(super::YEAR, 5)?)?;
    println!("The top crates are {}", part1_helper(input)?);
    Ok(())
}

fn part2() -> anyhow::Result<()> {
    let input = parse_input(parse_data_file(super::YEAR, 5)?)?;
    println!("The top crates are {}", part2_helper(input)?);
    Ok(())
}

fn part1_helper(input: Input) -> anyhow::Result<String> {
    helper(input, false)
}

fn part2_helper(input: Input) -> anyhow::Result<String> {
    helper(input, true)
}

fn helper(mut input: Input, in_order: bool) -> anyhow::Result<String> {
    for instruction in input.instructions {
        instruction.perform_move(&mut input.starting_stacks, in_order)?;
    }

    let mut tops = String::new();
    for mut stack in input.starting_stacks {
        if let Some(c) = stack.pop() {
            tops.push(c)
        }
    }

    Ok(tops)
}


fn parse_input(input: String) -> anyhow::Result<Input> {
    let mut stacks = Vec::new();
    let mut lines = input.lines();
    // start of the input is the stacks. Push them to the stacks "stack" (vector) until an
    // empty line.
    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            break
        } else {
            stacks.push(line);
        }
    }

    // last line for the stacks numbers the stacks and can be ignored. Using it to determine how
    // many stacks we need
    let mut starting_stacks = Vec::new();
    let num_stacks = stacks.pop().ok_or(anyhow::anyhow!("There was not a single stack in the input!"))?;
    let mut num_stacks_split = num_stacks.trim().split_whitespace();
    while let Some(_) = num_stacks_split.next() {
        starting_stacks.push(Vec::new());
    }

    // Now capture the stacks and push them to the starting stacks from bottum to top.
    while let Some(line) = stacks.pop() {
        // Each crate should be either "[<letter>] " or "    "
        let mut stack_index = 0;
        let mut chars = line.chars().peekable();
        while let Some(_) = chars.next() {
            // already took pulled the '[' character. Time to inspect this character.
            match chars.next() {
                // empty value
                Some(' ') => (),
                Some(c) => starting_stacks[stack_index].push(c),
                None => bail!("Expecting a character for stack index {}!", stack_index)
            }
            stack_index += 1;
            chars.next().ok_or(anyhow::anyhow!("Expecting to have a ']' character to take from the chars!"))?;
            // handle the spaces.
            if chars.peek().is_some() {
                chars.next();
            }
        }
    }

    Ok(Input {
        starting_stacks,
        // The remaining input should be Instructions.
        instructions: lines
            .map(|line| Instruction::from_str(line))
            .collect::<anyhow::Result<Vec<Instruction>>>()?
    })
}



#[cfg(test)]
mod test {
    use super::*;
    use crate::parse_test_data_file;

    #[test]
    fn parse_input_test() {
        let input_str = parse_test_data_file(super::super::YEAR, 5).expect("Failed to get test data from file!");
        let input = parse_input(input_str);

        assert!(input.is_ok());
        let input = input.unwrap();

        let expected_input = Input {
            starting_stacks: vec![
                vec!['Z', 'N'],
                vec!['M', 'C', 'D'],
                vec!['P']
            ],
            instructions: vec![
                Instruction { crates_to_move: 1, from: 2, to: 1 },
                Instruction { crates_to_move: 3, from: 1, to: 3 },
                Instruction { crates_to_move: 2, from: 2, to: 1 },
                Instruction { crates_to_move: 1, from: 1, to: 2 },
            ]
        };

        assert_eq!(expected_input, input);
    }

    #[test]
    fn part1_test() {
        let input_str = parse_test_data_file(super::super::YEAR, 5).expect("Failed to get test data from file!");
        let input = parse_input(input_str).unwrap();
        assert_eq!(String::from("CMZ"), part1_helper(input).expect("expect the test input to return a value!"));

    }

    #[test]
    fn part2_test() {
        let input_str = parse_test_data_file(super::super::YEAR, 5).expect("Failed to get test data from file!");
        let input = parse_input(input_str).unwrap();
        assert_eq!(String::from("MCD"), part2_helper(input).expect("expect the test input to return a value!"));

    }
}
