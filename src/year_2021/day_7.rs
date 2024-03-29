use crate::parse_data_file;

pub struct Day7 {}

impl Default for Day7 {
    fn default() -> Self {
        Self { }
    }
}

impl crate::DayAnswers for Day7 {
    fn get_answer(&self, _question: crate::model::Question) -> anyhow::Result<()> {
        main()
    }
}

fn main() -> anyhow::Result<()> {
    // let input_string = parse_data_file("test.txt")?;
    let data = parse_data_file(super::YEAR, 7)?;
    let mut crab_pos: Vec<i32> = data.split(',').into_iter()
        .map(|num_str| num_str.trim().parse().expect(&format!("Expecting a number from input string! {}", num_str)))
        .collect();
    crab_pos.sort();

    // Median. Needed for solution 1
    // let optimal_pos = if crab_pos.len() % 2 == 0 {
    //     let median_sum = crab_pos.get((crab_pos.len()/2) - 1).unwrap() + crab_pos.get(crab_pos.len()/2).unwrap();
    //     median_sum / 2
    // } else {
    //     *crab_pos.get(crab_pos.len() - 1).unwrap()
    // };

    // let mut answer = 0;
    // for crab in crab_pos {
    //     answer += (optimal_pos - crab).abs();
    // }

    // Avg. Need for solution 2
    let mut min = i32::MAX;
    let mut sum = 0;
    for crab in crab_pos.iter() {
        if *crab < min {
            min = *crab
        }

        sum += *crab;
    }

    let avg = sum / crab_pos.len() as i32;
    let optimal_pos = min + avg;
    println!("Optimal Position: {}", optimal_pos);

    let mut answer = 0;
    for crab in crab_pos {
        let difference = (crab - optimal_pos).abs();
        let range = 0..=difference;
        answer += range.sum::<i32>();
    }

    println!("The answer is {}", answer);

    Ok(())
}

/*
 * 0, 1, 1, 2, 2, 2, 4, 7, 14, 16
 */
