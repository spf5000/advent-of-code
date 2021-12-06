use std::path::PathBuf;

pub fn parse_data_file<T: AsRef<str>>(relative_path: T) -> std::io::Result<String> {
    let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    data_path.push("data");
    data_path.push(relative_path.as_ref());
    println!("Reading data from {}", data_path.display());
    std::fs::read_to_string(&data_path)
}
