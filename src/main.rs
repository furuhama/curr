use curl::easy::{Easy, List};
use serde::Deserialize;
use std::env;
use std::fs;
use std::io::{stdout, Write};

#[derive(Debug, Deserialize)]
struct CurlConfig {
    url: String,
    verbose: Option<bool>,
    show_header: Option<bool>,
    headers: Option<Vec<String>>,
    cookie: Option<String>,
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
    match conf.verbose {
        Some(v) => {
            easy.verbose(v).unwrap();
        },
        None => {},
    }
    match conf.show_header {
        Some(sh) => {
            easy.show_header(sh).unwrap();
        },
        None => {},
    }
    match conf.headers {
        Some(hs) => {
            let mut list = List::new();
            hs.iter().for_each(|h| list.append(&h).unwrap());
            easy.http_headers(list).unwrap();
        },
        None => {},
    }
    match conf.cookie {
        Some(c) => {
            easy.cookie(&c).unwrap();
        },
        None => {},
    }

    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    })
    .unwrap();
    easy.perform().unwrap();

    println!("{}", easy.response_code().unwrap());
}
