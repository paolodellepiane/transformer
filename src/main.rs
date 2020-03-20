use serde_json::Value;
use std::io::Seek;
use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read, SeekFrom},
    path::Path,
};

fn main() {
    let v = read_json_from_file("appsettings-bom.json").expect("error parsing JSON");
    let prefix = "";
    transform(&v, prefix).expect("error transforming");
}

fn transform(value: &Value, prefix: &str) -> Result<(), Box<dyn Error>> {
    for (n, v) in value.as_object().unwrap().iter() {
        let prefix = format!("{}{}", prefix, n);
        match v {
            Value::String(x) => println!("{} = {}", prefix, x),
            Value::Number(x) => println!("{} = {}", prefix, x),
            Value::Bool(x) => println!("{} = {}", prefix, x),
            Value::Object(_) => transform(v, &format!("{}__", &prefix)).unwrap(),
            _ => println!("unknown"),
        }
    }
    Ok(())
}

fn read_json_from_file<P: AsRef<Path>>(path: P) -> Result<Value, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buf = vec![0; 3];
    reader.read_exact(&mut buf)?;

    if !is_bom(&buf) {
        reader.seek(SeekFrom::Start(0))?;
    }

    let u: Value = serde_json::from_reader(reader)?;
    Ok(u)
}

fn is_bom(buf: &[u8]) -> bool {
    buf[0] == 0xef && buf[1] == 0xbb && buf[2] == 0xbf
}
