use std::path::PathBuf;
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    data_path.push("data/2.txt");

    let data = std::fs::read_to_string(&data_path)?;
    let mut increments = 0;
    let vals: Vec<i64> = data.lines()
        .map(|line| line.parse::<i64>())
        .filter(|val| {
            let output = val.is_ok();
            if !output { println!("Skipping val: {:?}", val); }
            output
        })
        .map(|val| val.unwrap())
        .collect();

    for i in 4..=vals.len() {
        let prev = vals[i-4..i-1].iter().fold(0, |sum, i| sum + i);
        let current = vals[i-3..i].iter().fold(0, |sum, i| sum + i);
        if current > prev { increments += 1; }
    }

    println!("Increments: {}", increments);

    Ok(())
}
