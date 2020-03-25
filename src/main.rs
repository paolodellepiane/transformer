use serde_json::Value;
use std::{
    error::Error,
    path::Path,
};

fn main() {
    let v = read_json_from_file("appsettings-bom.json").expect("error parsing JSON");
    transform(&v, &[])
}

fn transform(value: &Value, path: &[&str]) {
    let prefix = path.join("__");
    match value {
        Value::Object(o) => o.iter().for_each(|(n, v)| transform(v,  &[&path[..], &[n]].concat())),
        _ => println!("{} = {}", prefix, value.to_string())
    }
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
