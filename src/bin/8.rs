use advent_of_code::parse_data_file;

const DAYS: u16 = 256;

fn main() -> anyhow::Result<()> {
    // let input_string = parse_data_file("test.txt")?;
    let input_string = parse_data_file("8.txt")?;
    let mut output: u32 = 0;
    for line in input_string.lines() {
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
