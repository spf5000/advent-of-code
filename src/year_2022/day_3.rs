use anyhow::{bail, anyhow};

use crate::parse_data_file;
use std::collections::HashSet;

pub struct Day3 { }

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

fn part1() -> anyhow::Result<()> {
    let input = parse_data_file(super::YEAR, 3)?;
    let common = input.lines().map(|line| {
        let line = line.trim();
        let compartment1 = &line[0..line.len()/2];
        let compartment2 = &line[line.len()/2..line.len()];
        let campartment1_chars: HashSet<char> = compartment1.chars().collect();
        for c in compartment2.chars() {
            // assuming only a single shared char per comparment
            if campartment1_chars.contains(&c) {
                return Ok(c)
            }
        }
        bail!("Failed to find a common character in {} and {}", compartment1, compartment2);
    }).collect::<anyhow::Result<Vec<char>>>()?;

    let score = calculate_score(common)?;
    println!("Final score for problem 1: {}", score);

    Ok(())
}

fn part2() -> anyhow::Result<()> {
    let input = parse_data_file(super::YEAR, 3)?;
    let mut iter = input.lines().peekable();
    let mut common_chars = Vec::new();
    while iter.peek().is_some() {
        // Assuming that there will be 3 lines for each elf.
        let first = iter.next().ok_or_else(|| anyhow!("No sack for the first elf!"))?.chars().collect::<HashSet<char>>();
        let after_second = iter.next().ok_or_else(|| anyhow!("No sack for the second elf!"))?.chars()
            .filter(|c| first.contains(c))
            .collect::<HashSet<char>>();
        let after_third = iter.next().ok_or_else(|| anyhow!("No sack for the third elf!"))?.chars()
            .filter(|c| after_second.contains(c))
            .collect::<HashSet<char>>();
        if after_third.len() == 1 {
            common_chars.push(after_third.into_iter().next().expect("Expect a single value from the union of the elves sacks!"))
        } else {
            bail!("Found more than one common item (char) across the three elves! {:#?}", after_third);
        }
    }

    let score = calculate_score(common_chars)?;
    println!("Final score for problem 2: {}", score);

    Ok(())
}

fn get_score(c: char) -> anyhow::Result<u8> {
    if c.is_uppercase() {
        Ok((c as u8) - ('A' as u8) + 27)
    } else if c.is_lowercase() {
        Ok((c as u8) - ('a' as u8) + 1)
    } else {
        bail!("Provided non alphabetical character! {}", c)
    }
}

fn calculate_score(items: Vec<char>) -> anyhow::Result<u32> {
    let mut score = 0;
    for c in items {
        score += u32::from(get_score(c)?);
    }
    Ok(score)
}
