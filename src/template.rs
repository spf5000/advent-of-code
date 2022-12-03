use crate::parse_data_file;

pub struct DayX { }

impl Default for DayX {
    fn default() -> Self {
        Self { }
    }
}

impl crate::DayAnswers for DayX {
    fn get_answer(&self, question: crate::model::Question) -> anyhow::Result<()> {
        crate::run_question_answers(part1, part2, question)
    }
}

fn part1() -> anyhow::Result<()> {
    parse_data_file(super::YEAR, 0)?;
    Ok(())
}

fn part2() -> anyhow::Result<()> {
    parse_data_file(super::YEAR, 0)?;
    Ok(())
}
