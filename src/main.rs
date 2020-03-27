use clap::{crate_authors, crate_version, App, Arg};
use glob::glob;
use serde_json::Value;
use std::{error::Error, path::Path};

fn main() {
    let matches = App::new("transformer")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Transforms .net core settings json to docker env files")
        .arg(Arg::with_name("glob").short("g").default_value("**/*.json"))
        .get_matches();

    traverse(matches.value_of("glob").unwrap()).unwrap()
}

fn traverse(pattern: &str) -> Result<(), Box<dyn Error>> {
    for entry in glob(pattern)?.filter_map(Result::ok) {
        println!("// {:?}", entry);
        read_json_from_file(entry).ok().map(|v| transform(&v, &[]));
    }
    Ok(())
}

fn transform(value: &Value, path: &[&str]) {
    let prefix = path.join("__");
    match value {
        Value::Object(o) => o.iter().for_each(|(n, v)| transform(v, &append(path, n))),
        Value::Array(a) => a
            .iter()
            .enumerate()
            .for_each(|(i, v)| transform(v, &append(path, &i.to_string()))),
        _ => println!("{} = {}", prefix, value.to_string()),
    }
}

fn read_json_from_file<P: AsRef<Path>>(path: P) -> Result<Value, Box<dyn Error>> {
    let buf = std::fs::read(path)?;
    let v = serde_json::from_slice(strip_bom(&buf))?;
    Ok(v)
}

fn strip_bom(buf: &[u8]) -> &[u8] {
    let b = if is_bom(&buf) { &buf[3..] } else { &buf[..] };
    b
}

fn is_bom(buf: &[u8]) -> bool {
    buf.len() > 2 && buf[0] == 0xef && buf[1] == 0xbb && buf[2] == 0xbf
}

fn append<'a>(a: &[&'a str], b: &'a str) -> Vec<&'a str> {
    [a, &[b]].concat()
}
