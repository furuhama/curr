use curl::easy::Easy;
use serde::Deserialize;
use std::env;
use std::fs;
use std::io::{stdout, Write};

#[derive(Debug, Deserialize)]
struct CurlConfig {
    url: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("file not given");
        return;
    }

    let filename = &args[1];
    let content = match fs::read_to_string(filename) {
        Ok(c) => c,
        _ => {
            println!("invalid filename");
            return;
        }
    };

    let conf: CurlConfig = serde_yaml::from_str(&content).unwrap();

    let mut easy = Easy::new();
    easy.url(&conf.url).unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    })
    .unwrap();
    easy.perform().unwrap();

    println!("{}", easy.response_code().unwrap());
}
