use std::path::PathBuf;
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    data_path.push("data/1.txt");

    let data = std::fs::read_to_string(&data_path)?;
    let mut increments = 0;
    let mut data_lines = data.lines().peekable();
    while let Some(current) = data_lines.next() {
        if let Some(next) = data_lines.peek() {
            let current_num = current.parse::<i64>()?;
            let next_num = next.parse::<i64>()?;
            if next_num > current_num {
                increments += 1;
            }
        }
    }

    println!("Increments: {}", increments);

    Ok(())
}
