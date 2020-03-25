use serde_json::Value;
use std::{
    error::Error,
    path::Path,
};

fn main() {
    let v = read_json_from_file("appsettings-bom.json").expect("error parsing JSON");
    transform(&v, "").expect("error transforming");
}

fn transform(value: &Value, prefix: &str) -> Result<(), Box<dyn Error>> {
    for (n, v) in value.as_object().unwrap().iter() {
        let prefix = format!("{}{}", prefix, n);        
        match v {
            Value::Object(_) => transform(v, &format!("{}__", &prefix))?,
            _ => println!("{} = {}", prefix, v.to_string()),
        }
    }
    Ok(())
}

fn read_json_from_file<P: AsRef<Path>>(path: P) -> Result<Value, Box<dyn Error>> {
    let buf = std::fs::read(path)?;
    let v = serde_json::from_slice(strip_bom(&buf))?;
    Ok(v)
}

fn strip_bom(buf: &[u8]) -> &[u8] {
    if is_bom(&buf) { &buf[3..] } else { &buf[..] }
}

fn is_bom(buf: &[u8]) -> bool {
    buf.len() > 2 && buf[0] == 0xef && buf[1] == 0xbb && buf[2] == 0xbf
}
