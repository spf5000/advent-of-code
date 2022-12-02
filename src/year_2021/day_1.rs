use crate::parse_data_file;
use crate::model::Question;

pub struct Day1 { }

impl Default for Day1 {
    fn default() -> Self {
        Self { }
    }
}

impl crate::DayAnswers for Day1 {
    fn get_answer(&self, question: Question) -> anyhow::Result<()> {
        crate::run_question_answers(part1, part2, question)
    }
}

fn part1() -> anyhow::Result<()> {
    let data = parse_data_file(super::YEAR, 1)?;
    let mut increments = 0;
    let mut data_lines = data.lines().peekable();
    while let Some(current) = data_lines.next() {
        if let Some(next) = data_lines.peek() {
            let current_num = current.parse::<i64>()?;
            let next_num = next.parse::<i64>()?;
            if next_num > current_num {
                increments += 1;
            }
        }
    }

    println!("Increments: {}", increments);

    Ok(())
}

pub fn part2() -> anyhow::Result<()> {
    let data = parse_data_file(super::YEAR, 1)?;
    let mut increments = 0;
    let vals: Vec<i64> = data.lines()
        .map(|line| line.parse::<i64>())
        .filter(|val| {
            let output = val.is_ok();
            if !output { println!("Skipping val: {:?}", val); }
            output
        })
        .map(|val| val.unwrap())
        .collect();

    for i in 4..=vals.len() {
        let prev = vals[i-4..i-1].iter().fold(0, |sum, i| sum + i);
        let current = vals[i-3..i].iter().fold(0, |sum, i| sum + i);
        if current > prev { increments += 1; }
    }

    println!("Increments: {}", increments);

    Ok(())
}
