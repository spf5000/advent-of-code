use crate::parse_data_file;

pub struct Day8 {}

impl Default for Day8 {
    fn default() -> Self {
        Self { }
    }
}

impl crate::DayAnswers for Day8 {
    fn get_answer(&self, question: crate::model::Question) -> anyhow::Result<()> {
        crate::run_question_answers(part1, part2, question)
    }
}

fn part1() -> anyhow::Result<()> {
    // let input_string = parse_data_file("test.txt")?;
    let data = parse_data_file(super::YEAR, 8)?;
    let mut output: u32 = 0;
    for line in data.lines() {
        let split: Vec<&str> = line.split('|').collect();
        let answer = split[1];
        let numbers = answer.trim().split(char::is_whitespace);
        for number in numbers {
            match number.trim().len() {
                // 1, 4, 7, 8
                2 | 4 | 3 | 7 => {println!("Number: {}", number); output += 1},
                _ => ()
            };
        }
    }

    println!("The Answer is {}", output);

    Ok(())
}

fn part2() -> anyhow::Result<()> {
    // let input_string = parse_data_file("test.txt")?;
    let data = parse_data_file(super::YEAR, 8)?;
    let mut output: u32 = 0;
    for line in data.lines() {
        let split: Vec<&str> = line.split('|').collect();
        let answer = split[1];
        let numbers = answer.trim().split(char::is_whitespace);
        for number in numbers {
            match number.trim().len() {
                // 1, 4, 7, 8
                2 | 4 | 3 | 7 => {println!("Number: {}", number); output += 1},
                _ => ()
            };
        }
    }

    println!("The Answer is {}", output);

    Ok(())
}
