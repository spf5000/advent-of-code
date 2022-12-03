use std::str::FromStr;

use crate::parse_data_file;

pub struct Day2 { }

impl Default for Day2 {
    fn default() -> Self {
        Self { }
    }
}

impl crate::DayAnswers for Day2 {
    fn get_answer(&self, question: crate::model::Question) -> anyhow::Result<()> {
        crate::run_question_answers(part1, part2, question)
    }
}

fn part1() -> anyhow::Result<()> {
    let mut score = 0;

    let input = parse_input()?;
    for (opponent, mine) in input.into_iter() {
        let mine = RPS::from_str(&mine)?;
        score += opponent.calculate_score(&mine);
    }

    println!("Total Score for solution 1: {}", score);
    Ok(())
}

fn part2() -> anyhow::Result<()> {
    let mut score = 0;

    let input = parse_input()?;
    for (opponent, mine) in input.into_iter() {
        let mine = Stratz::from_str(&mine)?;
        let mine = mine.get_rps(&opponent);
        score += opponent.calculate_score(&mine);
    }

    println!("Total Score for solution 2: {}", score);
    Ok(())
}

fn parse_input() -> anyhow::Result<Vec<(RPS, String)>> {
    let input = parse_data_file(crate::model::Year::Year2022, 2)?;
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> anyhow::Result<(RPS, String)> {
    let mut split = line.split(" ");
    let opponent = RPS::from_str(split.next().ok_or_else(|| anyhow::anyhow!("Line does not contain the oppoent's play: {}", line))?)?;
    let mine = String::from(split.next().ok_or_else(|| anyhow::anyhow!("Line does not contain the my play: {}", line))?);
    Ok((opponent, mine))
}

enum RPS {
    Rock,
    Paper,
    Scissors
}

impl FromStr for RPS {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => anyhow::bail!("Provided a String that cannot be converted to a Rock, Paper, or Scissors by the rules! {}", s)
        }
    }
}

impl RPS {
    fn calculate_score(&self, mine: &RPS) -> u32 { 
        let mut score = match mine {
            RPS::Rock => 1,
            RPS::Paper=> 2,
            RPS::Scissors=> 3,
        };

        score += match (self, mine) {
            // Win
            (RPS::Rock, RPS::Paper) | (RPS::Paper, RPS::Scissors) | (RPS::Scissors, RPS::Rock) => 6,
            // Draw
            (RPS::Rock, RPS::Rock) | (RPS::Paper, RPS::Paper) | (RPS::Scissors, RPS::Scissors) => 3,
            // Lose
            (RPS::Rock, RPS::Scissors) | (RPS::Paper, RPS::Rock) | (RPS::Scissors, RPS::Paper) => 0,
        };

        score
    }
}

enum Stratz {
    Win,
    Draw,
    Lose
}

impl FromStr for Stratz {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => anyhow::bail!("Given strategy string that isn't supported! {}", s)
        }
    }
}

impl Stratz {
    fn get_rps(&self, opponent: &RPS) -> RPS {
        match (opponent, self) {
            (RPS::Rock, Self::Draw) | (RPS::Paper, Self::Lose) | (RPS::Scissors, Self::Win) => RPS::Rock,
            (RPS::Paper, Self::Draw) | (RPS::Scissors, Self::Lose) | (RPS::Rock, Self::Win) => RPS::Paper,
            (RPS::Scissors, Self::Draw) | (RPS::Rock, Self::Lose) | (RPS::Paper, Self::Win) => RPS::Scissors,
        }
    }
}
