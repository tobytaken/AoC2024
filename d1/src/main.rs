use std::fs::File;
use std::io::BufRead;
fn main() {
    let f = File::open("input.txt").expect("Failed to open input.txt");
    let content: Vec<String> = std::io::BufReader::new(f)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let split_numbers = split_numbers(&content);

    one(&split_numbers);
    two(&split_numbers);
}

fn one(split_numbers: &(Vec<i32>, Vec<i32>)) {
    let (left, right) = split_numbers;
    let mut sum: i32 = 0;

    for (i, number) in left.iter().enumerate() {
        let diff = number - right[i];
        sum += diff.abs();
    }

    println!("{:#?}", sum);
}

fn two(split_numbers: &(Vec<i32>, Vec<i32>)) {
    let (left, right) = split_numbers;
    let mut sum: i32 = 0;

    for number_left in left.iter() {
        let mut matches = 0;

        for number_right in right.iter() {
            if number_left == number_right {
                matches += 1;
            }
        }
        sum += number_left * matches;
    }
    println!("{}", sum);
}

fn split_numbers(content: &Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    let mut sum: i32 = 0;
    for line in content {
        for number in line.split_whitespace().enumerate() {
            if (number.0 == 0) {
                left.push(number.1.parse().unwrap());
            } else {
                right.push(number.1.parse().unwrap());
            }
        }
    }
    left.sort_unstable();
    right.sort_unstable();

    (left, right)
}
