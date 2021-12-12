use advent_of_code::parse_data_file;

const DAYS: u16 = 256;

fn main() -> anyhow::Result<()> {
    // let input_string = parse_data_file("test.txt")?;
    let input_string = parse_data_file("9.txt")?;
    let mut output = 0;
    let mut output_2: Vec<u128> = Vec::new();
    for line in input_string.lines() {
        let mut stack = Vec::new();
        let mut incomplete = true;
        for c in line.chars() {
            match c {
                '[' | '(' | '<' | '{' => stack.push(c),
                ')' => {
                    if !is_closed('(', stack.pop()) {
                        output += 3;
                        incomplete = false;
                        break;
                    }
                }
                ']' => {
                    if !is_closed('[', stack.pop()) {
                        output += 57;
                        incomplete = false;
                        break;
                    }
                }
                '}' => {
                    if !is_closed('{', stack.pop()) {
                        output += 1197;
                        incomplete = false;
                        break;
                    }
                }
                '>' => {
                    if !is_closed('<', stack.pop()) {
                        output += 25137;
                        incomplete = false;
                        break;
                    }
                }
                _ => panic!("Unexpected input character: {}", c),
            };
        }

        if !incomplete {
            println!("Invalid line: {}", line);
            continue;
        }

        // remaining characters in the stack are characters to complete the stack.
        let mut incomplete_output = 0;
        while let Some(open) = stack.pop() {
            incomplete_output *= 5;
            incomplete_output += match open {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!("Unexpected open character: {}", open),
            };
        }
        output_2.push(incomplete_output);
    }

    println!("The First Answer is {}", output);

    output_2.sort();
    println!("The Second Answer is {}", output_2[output_2.len() / 2]);

    Ok(())
}

fn is_closed(open_needed: char, previous_open: Option<char>) -> bool {
    match previous_open {
        Some(previous) => open_needed == previous,
        None => false,
    }
}
