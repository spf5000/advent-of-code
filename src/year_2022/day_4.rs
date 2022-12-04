use anyhow::bail;

use crate::parse_data_file;
use std::ops::RangeInclusive;
use std::str::FromStr;

pub struct Day4 { }

impl Default for Day4 {
    fn default() -> Self {
        Self { }
    }
}

impl crate::DayAnswers for Day4 {
    fn get_answer(&self, question: crate::model::Question) -> anyhow::Result<()> {
        crate::run_question_answers(part1, part2, question)
    }
}

fn part1() -> anyhow::Result<()> {
    let mut intersections = 0;
    for (first, second) in parse_input()? {
        if first.contains(&second) || second.contains(&first) {
            intersections += 1;
        }
    }
    println!("Found {} Intersections!", intersections);

    Ok(())
}

fn part2() -> anyhow::Result<()> {
    let mut overlaps = 0;
    for (first, second) in parse_input()? {
        if first.overlaps(&second) || second.overlaps(&first) {
            overlaps += 1;
        }
    }
    println!("Found {} Overlaps!", overlaps);

    Ok(())
}

fn parse_input() -> anyhow::Result<Vec<(Section, Section)>> {
    let input = parse_data_file(super::YEAR, 4)?;
    let mut output = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let (first, second) = split_into_double(&line, ',')?;
        output.push((parse_section(i.try_into()?, first)?, parse_section(i.try_into()?, second)?));
    }
    Ok(output)
}

fn parse_section<T: AsRef<str>>(elf: u32, section: T) -> anyhow::Result<Section> {
    let section = split_into_double(&section, '-')?;
    Section::try_from((elf, section))
}

fn split_into_double<T: AsRef<str>>(input: &T, split_char: char) -> anyhow::Result<(&str, &str)> {
    let mut split = input.as_ref().split(split_char);
    let start = split.next().ok_or(anyhow::anyhow!("Split didn't include the first part of the split! {}", input.as_ref()))?;
    let end = split.next().ok_or(anyhow::anyhow!("Split didn't include the second part of the split! {}", input.as_ref()))?;
    if let Some(_) = split.next() {
        bail!("Split contained three parts, not just 2! {}", input.as_ref())
    } else {
        Ok((start, end))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Section {
    section: RangeInclusive<u8>,
    elf: u32,
}

impl Section {
    fn contains(&self, other: &Self) -> bool {
        self.section.start() <= other.section.start() && self.section.end() >= other.section.end()
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.section.contains(other.section.start()) || self.section.contains(other.section.end())
    }
}

impl TryFrom<(u32, (&str, &str))> for Section {
    type Error = anyhow::Error;
    fn try_from(value: (u32, (&str, &str))) -> Result<Self, Self::Error> {
        Ok(Self {
            elf: value.0,
            section: u8::from_str(value.1.0)?..=u8::from_str(value.1.1)?
        })
    }
}
