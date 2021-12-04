use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    data_path.push("data/2.txt");

    let data = std::fs::read_to_string(&data_path)?;
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
