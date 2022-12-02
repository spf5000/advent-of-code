use crate::{parse_data_file, DayAnswers};
use crate::model::Question;

pub struct Day2 { }

impl Default for Day2 {
    fn default() -> Self {
        Self { }
    }
}

impl DayAnswers for Day2 {
    fn get_answer(&self, _question: Question) -> anyhow::Result<()> {
        Self::main()
    }
}

impl Day2 {
    fn main() -> anyhow::Result<()> {
        let data = parse_data_file(super::YEAR, 2)?;
        let mut horizontal = 0;
        let mut depth = 0;
        let mut aim = 0;
        for line in data.lines() {
            match line {
                s if s.starts_with("forward ") => {
                    let val = s["forward ".len()..].parse::<i64>()?;
                    horizontal += val;
                    depth += val * aim;
                },
                s if s.starts_with("down ") => {
                    let val = s["down ".len()..].parse::<i64>()?;
                    aim += val;
                },
                s if s.starts_with("up ") => {
                    let val = s["up ".len()..].parse::<i64>()?;
                    aim -= val;
                },
                s => {
                    println!("Unexpected line: {}", s);
                }
            }
        }

        println!("Answer: {}", &depth * &horizontal);

        Ok(())
    }
}
