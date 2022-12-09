// #![feature(map_first_last)] TODO: This feature was merged into Beta, so the project is using
// beta. Should update to stable once it reaches stable.
pub mod year_2021;
pub mod year_2022;
pub mod model;

use std::path::PathBuf;

use model::Question;

pub(crate) fn parse_data_file(year: model::Year, day: u8) -> std::io::Result<String> {
    parse_data_file_helper(year, day, false)
}

#[cfg(test)]
pub(crate) fn parse_test_data_file(year: model::Year, day: u8) -> std::io::Result<String> {
    parse_data_file_helper(year, day, true)
}

fn parse_data_file_helper(year: model::Year, day: u8, is_test: bool) -> std::io::Result<String> {
    let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    data_path.push("input");
    data_path.push(year.to_string());
    if is_test {
        data_path.push(format!("{}-test", day));
    } else {
        data_path.push(day.to_string());
    }
    data_path.set_extension("txt");
    println!("Reading data from {}", data_path.display());
    std::fs::read_to_string(&data_path)
}

pub(crate) fn run_question_answers<F, S>(first_answer:F, second_answer: S, desired_answers: Question) -> anyhow::Result<()> 
where F: Fn() -> anyhow::Result<()>,
S: Fn() -> anyhow::Result<()>
{
    match desired_answers {
        Question::First => first_answer(),
        Question::Second => second_answer(),
        Question::Both => first_answer().and(second_answer())
    }
}

pub trait DayAnswers {
    // TODO: Consider actually making this an output
    fn get_answer(&self, question: Question) -> anyhow::Result<()>;
}

pub trait YearAnswers {
    fn get_answers_for_day(&self, day: u8) -> anyhow::Result<Box<dyn DayAnswers>>; 
}
