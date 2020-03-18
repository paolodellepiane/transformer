use serde_json::Value;
use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

fn main() {
    let json: Value = read_json_from_file("appsettings.json").expect("JSON was not well-formatted");
    println!("{}", json.as_object().unwrap().len());
}

fn read_json_from_file<P: AsRef<Path>>(path: P) -> Result<Value, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let u: Value = serde_json::from_reader(reader)?;
    Ok(u)
}

fn read_json_from_file2<P: AsRef<Path>>(path: P) -> Result<Value, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let u: Value = serde_json::from_str(&contents)?;
    Ok(u)
}
