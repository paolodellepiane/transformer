use std::{path::Path, fs::File, io::{Read, BufReader}, error::Error};
use serde_json::{Value};

fn main() {
    let json: Value = read_json_from_file("C:\\Users\\pdell\\w\\rs\\transformer\\appsettings.json").expect("JSON was not well-formatted");    
    println!("{:?}", json.as_object().unwrap().len());
}

fn read_json_from_file<P: AsRef<Path>>(path: P) -> Result<Value, Box<dyn Error>> {
    let pippo = "{
    \"I\": false,
}";

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let reader = BufReader::new(file);
    let u: Value = serde_json::from_str(&contents)?;
    Ok(u)
}