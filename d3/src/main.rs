use regex::Regex;
use std::fs::File;
use std::io::BufRead;

fn main() {
    let f = File::open("example.txt").expect("Failed to open input.txt");
    let content: Vec<String> = std::io::BufReader::new(f)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    one(&content);
}

fn one(content: &Vec<String>) {
    let reg = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let mut results: Vec<String> = vec![];

    for line in content {
        let Some(caps) = reg.captures(&line) else {
            println!("what");
            return;
        };

        for cap in caps.iter() {
            println!("{:#?}", cap);
        }
    }
}
