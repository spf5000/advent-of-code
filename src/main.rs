use clap::Parser;
use advent_of_code::model::{Year, Question};
use advent_of_code::YearAnswers;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Year of Advent of code being executed.
    #[arg(short, long, value_parser)]
    year: Year,

    /// Dya of Advent of code being executed. Should be between 1st - 25th (of December).
    #[arg(short, long)]
    day: u8,

    /// Year of Advent of code being executed.
    #[arg(short, long, value_parser, default_value_t = Question::Both)]
    question: Question,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.year {
        Year::Year2021 => {
            advent_of_code::year_2021::Year2021::default().get_answers_for_day(cli.day)?.get_answer(cli.question)
        },
        Year::Year2022 => {
            advent_of_code::year_2022::Year2022::default().get_answers_for_day(cli.day)?.get_answer(cli.question)
        }
    }
}
