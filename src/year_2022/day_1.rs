use crate::parse_data_file;
use std::collections::BTreeSet;
use std::str::FromStr;

pub struct Day1 { }

impl Default for Day1 {
    fn default() -> Self {
        Self { }
    }
}

impl crate::DayAnswers for Day1 {
    fn get_answer(&self, question: crate::model::Question) -> anyhow::Result<()> {
        crate::run_question_answers(part1, part2, question)
    }
}

fn part1() -> anyhow::Result<()> {
    let mut cals_per_elf = get_cals_per_elf()?;

    println!("Max Calories by a single elf: {}", cals_per_elf.pop_last().ok_or_else(|| {
        anyhow::anyhow!("No calories were added to the set (and presumably there are no elves")
    })?);

    Ok(())
}

fn part2() -> anyhow::Result<()> {
    let mut cals_per_elf = get_cals_per_elf()?;

    let mut answer = 0;
    for _ in 0..3 {
        answer += cals_per_elf.pop_last().unwrap_or(0);
    }

    println!("Calories by 3 elves: {}", answer);

    Ok(())

}

fn get_cals_per_elf() -> anyhow::Result<BTreeSet<u32>> {
    let input = parse_data_file(super::YEAR, 1)?;
    let mut cals_per_elf = BTreeSet::new();
    let mut current_cals: u32 = 0;

    for line in input.lines() {
        // more calories for the same elf.
        if let Ok(cals) = u32::from_str(line.trim()) {
            current_cals += cals;
        } 
        // NOTE: Assuming that if we can't parse a u32 out of the input it's an empty line (and
        // therefore a new elf)
        else {
            cals_per_elf.insert(current_cals);
            current_cals = 0;
        }
    }

    Ok(cals_per_elf)
}
